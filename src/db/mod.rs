use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

////////////////////////////////////////////////////////////////////////////////

pub(crate) type ConnectionPool = Arc<Pool<ConnectionManager<PgConnection>>>;

pub(crate) fn create_pool(url: &str, size: u32, timeout: u64) -> ConnectionPool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool = Pool::builder()
        .max_size(size)
        .connection_timeout(Duration::from_secs(timeout))
        .build(manager)
        .expect("Error creating a database pool");

    Arc::new(pool)
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromSqlRow, AsExpression)]
#[sql_type = "sql::Bucket"]
pub(crate) struct Bucket {
    label: String,
    audience: String,
}

impl Bucket {
    pub(crate) fn new(label: &str, audience: &str) -> Self {
        Self {
            label: label.to_owned(),
            audience: audience.to_owned(),
        }
    }

    pub(crate) fn audience(&self) -> &str {
        &self.audience
    }
}

impl fmt::Display for Bucket {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}.{}", self.label, self.audience)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromSqlRow, AsExpression)]
#[sql_type = "sql::Set"]
pub(crate) struct Set {
    label: String,
    bucket: Bucket,
}

impl Set {
    pub(crate) fn new(label: &str, bucket: Bucket) -> Self {
        Self {
            label: label.to_owned(),
            bucket,
        }
    }

    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    pub(crate) fn bucket(&self) -> &Bucket {
        &self.bucket
    }
}

impl fmt::Display for Set {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}:{}", self.bucket, self.label)
    }
}

////////////////////////////////////////////////////////////////////////////////

pub mod sql {
    use diesel::deserialize::{self, FromSql};
    use diesel::pg::Pg;
    use diesel::serialize::{self, Output, ToSql, WriteTuple};
    use diesel::sql_types::{Record, Text};
    use std::io::Write;

    #[derive(SqlType, QueryId)]
    #[postgres(type_name = "bucket")]
    #[allow(non_camel_case_types)]
    pub struct Bucket;

    impl ToSql<Bucket, Pg> for super::Bucket {
        fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
            WriteTuple::<(Text, Text)>::write_tuple(&(&self.label, &self.audience), out)
        }
    }

    impl FromSql<Bucket, Pg> for super::Bucket {
        fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
            let (label, audience): (String, String) =
                FromSql::<Record<(Text, Text)>, Pg>::from_sql(bytes)?;
            Ok(super::Bucket::new(&label, &audience))
        }
    }

    #[derive(SqlType, QueryId)]
    #[postgres(type_name = "set")]
    #[allow(non_camel_case_types)]
    pub struct Set;

    impl ToSql<Set, Pg> for super::Set {
        fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
            WriteTuple::<(Text, Bucket)>::write_tuple(&(&self.label, &self.bucket), out)
        }
    }

    impl FromSql<Set, Pg> for super::Set {
        fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
            let (label, bucket): (String, super::Bucket) =
                FromSql::<Record<(Text, Bucket)>, Pg>::from_sql(bytes)?;
            Ok(super::Set::new(&label, bucket))
        }
    }
}

pub(crate) mod tag;
