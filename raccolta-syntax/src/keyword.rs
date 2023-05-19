// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use strum::{
    AsRefStr,
    EnumIter,
};

/// A keyword is a reserved identifier which has semantic value that describes
/// the query.
///
/// This list currently contains SQL 1999 and MariaDB reserved identifiers. It
/// does not, in any way, describe the capabilities of this library. Some
/// keywords are used, but most are just reserved for compatibility and future
/// purposes.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter, strum::Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Keyword {
    Absolute,
    Accessible,
    Account,
    Action,
    Add,
    Admin,
    After,
    Against,
    Aggregate,
    Algorithm,
    Alias,
    All,
    Allocate,
    Alter,
    Always,
    Analyze,
    And,
    Any,
    Are,
    Array,
    As,
    /// Ascending
    Asc,
    Ascii,
    Asensitive,
    Assertion,
    At,
    Atomic,
    Authorization,
    Authors,
    AutoIncrement,
    Auto,
    AutoextendSize,
    Avg,
    AvgRowLength,

    Backup,
    Before,
    Begin,
    Between,
    Bigint,
    Binary,
    Binlog,
    Bit,
    Blob,
    Block,
    Body,
    Bool,
    Boolean,
    Both,
    Breadth,
    Btree,
    By,
    Bute,

    Cache,
    Call,
    Cascade,
    Cascaded,
    Case,
    Cast,
    Catalog,
    CatalogName,
    Chain,
    Change,
    Changed,
    Char,
    Character,
    Charset,
    Check,
    Checkpoint,
    Checksum,
    Cipher,
    Class,
    ClassOrigin,
    Client,
    Clob,
    Close,
    Coalesce,
    Code,
    Collate,
    Collation,
    Column,
    ColumnAdd,
    ColumnCheck,
    ColumnCreate,
    ColumnDelete,
    ColumnGet,
    ColumnName,
    Columns,
    Comment,
    Commit,
    Committed,
    Compact,
    Completion,
    Compressed,
    Concurrent,
    Condition,
    Consistent,
    Connect,
    Connection,
    Constraint,
    ConstraintCatalog,
    ConstraintName,
    ConstraintSchema,
    Constraints,
    Constructor,
    Contains,
    Context,
    Continue,
    Contributors,
    Convert,
    Corresponding,
    Count,
    Cpu,
    Create,
    Cross,
    Cube,
    Current,
    CurrentDate,
    CurrentPath,
    CurrentPos,
    CurrentRole,
    CurrentTime,
    CurrentTimestamp,
    CurrentUser,
    Cursor,
    CursorName,
    Cycle,

    Data,
    Database,
    Databases,
    Datafile,
    Date,
    Datetime,
    Day,
    DayHour,
    DayMicrosecond,
    DayMinute,
    DaySecond,
    Deallocate,
    Dec,
    Decimal,
    Declare,
    Default,
    Deferrable,
    Deferred,
    Definer,
    Delayed,
    DelayKeyWrite,
    Delete,
    DeleteDomainId,
    Depth,
    Deref,
    /// Descending
    Desc,
    Describe,
    DesKeyFile,
    Descriptor,
    Destroy,
    Destructor,
    Deterministic,
    Diagnostics,
    Dictionary,
    Directory,
    Disable,
    Discard,
    Disconnect,
    Disk,
    Distinct,
    Distinctrow,
    Div,
    Do,
    DoDomainIds,
    Domain,
    Double,
    Drop,
    Dual,
    Dumpfile,
    Duplicate,
    Dynamic,

    Each,
    Else,
    Elseif,
    Elsif,
    Empty,
    Enable,
    Enclosed,
    End,
    Ends,
    /// END-EXEC
    #[strum(to_string="END-EXEC")]
    EndExec,
    Engine,
    Engines,
    Enum,
    Equals,
    Error,
    Errors,
    Escape,
    Escaped,
    Event,
    Events,
    Every,
    Examined,
    Except,
    Exception,
    Exchange,
    Exclude,
    Exec,
    Execute,
    Exists,
    Exit,
    Expansion,
    Expire,
    Export,
    Extended,
    ExtentSize,
    External,

    False,
    Fast,
    Faults,
    Federated,
    Fetch,
    Fields,
    File,
    First,
    Fixed,
    Float,
    Float4,
    Float8,
    Flush,
    Following,
    Follows,
    For,
    Force,
    Foreign,
    Format,
    Found,
    From,
    Free,
    Full,
    Fulltext,
    Function,

    General,
    Generated,
    Get,
    GetFormat,
    Global,
    Go,
    Goto,
    Grant,
    Grants,
    Group,
    Grouping,

    Handler,
    Hard,
    Hash,
    Having,
    Help,
    HighPriority,
    History,
    Host,
    Hosts,
    Hour,
    HourMicrosecond,
    HourMinute,
    HourSecond,

    Id,
    Identified,
    Identity,
    If,
    Ignore,
    Ignored,
    IgnoreDomainIds,
    IgnoreServerIds,
    Immediate,
    Import,
    In,
    Increment,
    Index,
    Indexes,
    Indicator,
    Infile,
    Initialize,
    Initially,
    InitialSize,
    Inner,
    Inout,
    Input,
    Insensitive,
    Insert,
    InsertMethod,
    Install,
    Int,
    Int1,
    Int2,
    Int3,
    Int4,
    Int8,
    Integer,
    Intersect,
    Interval,
    Into,
    Invisible,
    Io,
    IoThread,
    Ipc,
    Is,
    Isolation,
    Isopen,
    Issuer,
    Iterate,
    Invoker,

    Join,
    Json,
    JsonTable,

    Key,
    Keys,
    KeyBlockSize,
    Kill,

    Language,
    Large,
    Last,
    LastValue,
    Lastval,
    Lateral,
    Leading,
    Leave,
    Leaves,
    Left,
    Less,
    Level,
    Like,
    Limit,
    Linear,
    Lines,
    List,
    Load,
    Local,
    Localtime,
    Localtimestamp,
    Locator,
    Lock,
    Locked,
    Locks,
    Logfile,
    Logs,
    Long,
    Longblob,
    Longtext,
    Loop,
    LowPriority,

    Map,
    //...
    // MariaDB master-slave config
    Match,
    MaxConnectionsPerHour,
    MaxQueriesPerHour,
    MaxRows,
    MaxSize,
    MaxStatementTime,
    MaxUpdatesPerHour,
    MaxUserConnections,
    Maxvalue,
    Medium,
    Mediumblob,
    Mediumint,
    Mediumtext,
    Memory,
    Merge,
    MessageText,
    Microsecond,
    Middleint,
    Migrate,
    Minus,
    Minute,
    MinuteMicrosecond,
    MinuteSecond,
    Minvalue,
    MinRows,
    Mod,
    Mode,
    Modifies,
    Modify,
    Module,
    Monitor,
    Month,
    Mutext,
    Mysql,
    MysqlErrno,

    Name,
    Names,
    National,
    Natural,
    Nchar,
    Nclob,
    Nested,
    Never,
    New,
    Next,
    Nextval,
    No,
    Nocache,
    Nocycle,
    Nodegroup,
    Nomaxvalue,
    Nominvalue,
    None,
    NoWait,
    Nowait,
    Not,
    Notfound,
    NoWriteToBinlog,
    Null,
    Number,
    Numeric,
    Nvarchar,

    Object,
    Of,
    Off,
    Offset,
    Old,
    OldPassword,
    On,
    One,
    Online,
    Only,
    Open,
    Operation,
    Optimize,
    Option,
    Optionally,
    Options,
    Or,
    Order,
    Ordinality,
    Others,
    Out,
    Outer,
    Outfile,
    Output,
    Over,
    Overlaps,
    Owner,

    Package,
    PackKeys,
    Pad,
    Page,
    Parameter,
    Parameters,
    Parser,
    ParseVcolExpr,
    Partial,
    Partition,
    Partitioning,
    Partitions,
    Password,
    Path,
    Period,
    Persistent,
    Phase,
    Plugin,
    Plugins,
    Port,
    Portion,
    Postfix,
    Precedes,
    Preceding,
    Precision,
    Prefix,
    Preorder,
    Prepare,
    Preserve,
    Prev,
    Previous,
    Primary,
    Prior,
    Privileges,
    Procedure,
    Process,
    Processlist,
    Profile,
    Profiles,
    Proxy,
    Public,
    Purge,

    Quarter,
    Query,
    Quick,

    Raise,
    Range,
    Raw,
    Read,
    ReadOnly,
    Reads,
    ReadWrite,
    Real,
    Rebuild,
    Recover,
    Recursive,
    RedoBufferSize,
    Redofile,
    Redundant,
    Ref,
    References,
    Referencing,
    RefSystemId,
    Regexp,
    Relative,
    Relay,
    Relaylog,
    RelayLogFile,
    RelayLogPos,
    RelayThread,
    Release,
    Reload,
    Remove,
    Rename,
    Reorganize,
    Repair,
    Repeat,
    Repeatable,
    Replace,
    Replay,
    Replica,
    ReplicaPos,
    Replicas,
    Replication,
    Require,
    Reset,
    Resignal,
    Restart,
    Restore,
    Restrict,
    Resume,
    Result,
    Return,
    ReturnedSqlstate,
    Returns,
    Reuse,
    Reverse,
    Revoke,
    Right,
    Rlike,
    Role,
    Rollback,
    Rollup,
    Routine,
    Row,
    RowCount,
    Rowcount,
    RowFormat,
    Rownum,
    Rows,
    Rowtype,
    Rtree,

    Savepoint,
    Schedule,
    Schema,
    SchemaName,
    Schemas,
    Scroll,
    Scope,
    Search,
    Second,
    SecondMicrosecond,
    Section,
    Security,
    Select,
    Sensitive,
    Separator,
    Sequence,
    Serial,
    Serializable,
    Session,
    SessionUser,
    Server,
    Set,
    Sets,
    Setval,
    Share,
    Show,
    Shutdown,
    Signal,
    Signed,
    Simple,
    Size,
    Skip,
    Slave,
    SlavePos,
    Slaves,
    Slow,
    Smallint,
    Snapshot,
    Socket,
    Soft,
    Some,
    Soname,
    Sounds,
    Source,
    Space,
    Spatial,
    Specific,
    Sql,
    Sqlexception,
    Sqlstate,
    Sqlwarning,
    SqlBigResult,
    SqlBufferResult,
    SqlCache,
    SqlCalcFoundRows,
    SqlNoCache,
    SqlSmallResult,
    SqlThread,
    SqlTsiSecond,
    SqlTsiMinute,
    SqlTsiHour,
    SqlTsiDay,
    SqlTsiWeek,
    SqlTsiMonth,
    SqlTsiQuarter,
    SqlTsiYear,
    Ssl,
    Stage,
    Start,
    Starting,
    Starts,
    State,
    Statement,
    Static,
    StatsAutoRecalc,
    StatsPersistent,
    StatsSamplePages,
    Status,
    Stop,
    Storage,
    Stored,
    StraightJoin,
    String,
    Structure,
    SubclassOrigin,
    Subject,
    Subpartition,
    Subpartitions,
    Super,
    Suspend,
    Swaps,
    Switches,
    Sysdate,
    System,
    SystemTime,
    SystemUser,

    Table,
    TableChecksum,
    TableName,
    Tables,
    Tablespace,
    Temporary,
    Temptable,
    Terminate,
    Terminated,
    Text,
    Than,
    Then,
    Threads,
    Ties,
    Time,
    Timestamp,
    Timestampadd,
    Timestampdiff,
    TimezoneHour,
    TimezoneMinute,
    Tinyblob,
    Tinyint,
    Tinytext,
    To,
    Trailing,
    Transaction,
    Transactional,
    Translation,
    Treat,
    Trigger,
    Triggers,
    True,
    Truncate,
    Type,
    Types,

    Unbounded,
    Uncommitted,
    Undefined,
    Under,
    Undo,
    UndoBufferSize,
    Undofile,
    Unicode,
    Union,
    Unique,
    Unknown,
    Unlock,
    Uninstall,
    Unnest,
    Unsigned,
    Until,
    Update,
    Upgrade,
    Usage,
    Use,
    UseFrm,
    User,
    UserResources,
    Using,
    UtcDate,
    UtcTime,
    UtcTimestamp,

    Value,
    Values,
    Varbinary,
    Varchar,
    Varcharacter,
    Varchar2,
    Variable,
    Variables,
    Varying,
    Versioning,
    Via,
    View,
    Virtual,
    Visible,

    Wait,
    Week,
    WeightString,
    When,
    Whenever,
    Where,
    While,
    Window,
    With,
    Within,
    Without,
    Work,
    Wrapper,
    Write,

    X509,
    Xor,
    Xa,
    Xml,

    Year,
    YearMonth,

    Zone,
    Zerofill,
}

/// Non-reserved words (`<non-reserved word>`) are words that are allowed in
/// some context as identifiers, and others as keywords. This is to avoid
/// clashing with commonly used column or table names such as `NUMBER`.
///
/// SQL-86 didn't have these `<non-reserved words>`, as they were introduced in
/// SQL-1992.
///
/// # About
/// **Specification:** SQL 1992, SQL 1999
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter, strum::Display)]
#[non_exhaustive]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum NonReservedWord {
    /// # About
    /// **Specification:** SQL 2003
    A,

    /// # About
    /// **Specification:** SQL 1999
    Abs,

    /// # About
    /// **Specification:** SQL 2003
    Absolute,

    /// # About
    /// **Specification:** SQL 2003
    Action,

    /// # About
    /// **Specification:** SQL 1992
    Ada,

    /// # About
    /// **Specification:** SQL 2016
    Add,

    /// # About
    /// **Specification:** SQL 2003
    Admin,

    /// # About
    /// **Specification:** SQL 2003
    After,

    /// # About
    /// **Specification:** SQL 2003
    Always,

    /// # About
    /// **Specification:** SQL 2003
    Asc,

    /// TODO check this according to SQL 2016
    /// # About
    /// **Specification:** SQL 1999
    Asensitive,

    /// # About
    /// **Specification:** SQL 2003
    Assertion,

    /// # About
    /// **Specification:** SQL 1999
    Assignment,

    /// TODO check this according to SQL 2016
    /// # About
    /// **Specification:** SQL 1999
    Asymmetric,

    /// TODO check this according to SQL 2016
    /// # About
    /// **Specification:** SQL 1999
    Atomic,

    /// # About
    /// **Specification:** SQL 2003
    Attribute,

    /// # About
    /// **Specification:** SQL 2003
    Attributes,

    /// TODO check this according to SQL 2016
    /// # About
    /// **Specification:** SQL 1999
    Avg,


    /// # About
    /// **Specification:** SQL 2003
    Before,

    /// # About
    /// **Specification:** SQL 2003
    Bernoulli,

    /// # About
    /// **Specification:** SQL 1999
    Between,

    /// # About
    /// **Specification:** SQL 1999
    BitLength,

    /// # About
    /// **Specification:** SQL 1999
    Bitvar,

    /// # About
    /// **Specification:** SQL 2003
    Breadth,


    /// # About
    /// **Specification:** SQL 1992
    C,

    /// # About
    /// **Specification:** SQL 1999
    Called,

    /// # About
    /// **Specification:** SQL 1999
    Cardinality,

    /// # About
    /// **Specification:** SQL 2003
    Cascade,

    /// # About
    /// **Specification:** SQL 2003
    Catalog,

    /// # About
    /// **Specification:** SQL 1992
    CatalogName,

    /// # About
    /// **Specification:** SQL 1999
    Chain,

    /// # About
    /// **Specification:** SQL 2016
    Chaining,

    /// # About
    /// **Specification:** SQL 1999
    CharLength,

    /// # About
    /// **Specification:** SQL 1999
    CharacterLength,

    /// # About
    /// **Specification:** SQL 1992
    CharacterSetCatalog,

    /// # About
    /// **Specification:** SQL 1992
    CharacterSetName,

    /// # About
    /// **Specification:** SQL 1992
    CharacterSetSchema,

    /// # About
    /// **Specification:** SQL 2003
    Characteristics,

    /// # About
    /// **Specification:** SQL 2003
    Characters,

    /// # About
    /// **Specification:** SQL 1999
    Checked,

    /// # About
    /// **Specification:** SQL 1992
    ClassOrigin,

    /// # About
    /// **Specification:** SQL 1999
    Coalesce,

    /// # About
    /// **Specification:** SQL 1992
    Cobol,

    /// # About
    /// **Specification:** SQL 2003
    Collation,

    /// # About
    /// **Specification:** SQL 1992
    CollationCatalog,

    /// # About
    /// **Specification:** SQL 1992
    CollationName,

    /// # About
    /// **Specification:** SQL 1992
    CollationSchema,

    /// # About
    /// **Specification:** SQL 1992
    ColumnName,

    /// # About
    /// **Specification:** SQL 2016
    Columns,

    /// # About
    /// **Specification:** SQL 1992
    CommandFunction,

    /// # About
    /// **Specification:** SQL 1999
    CommandFunctionCode,

    /// # About
    /// **Specification:** SQL 1992
    Committed,

    /// # About
    /// **Specification:** SQL 2016
    Conditional,

    /// # About
    /// **Specification:** SQL 1992
    ConditionNumber,

    /// # About
    /// **Specification:** SQL 2016
    Connection,

    /// # About
    /// **Specification:** SQL 1992
    ConnectionName,

    /// # About
    /// **Specification:** SQL 1992
    ConstraintCatalog,

    /// # About
    /// **Specification:** SQL 1992
    ConstraintName,

    /// # About
    /// **Specification:** SQL 1992
    ConstraintSchema,

    /// # About
    /// **Specification:** SQL 2003
    Constraints,

    /// # About
    /// **Specification:** SQL 2016
    Constructor,

    /// # About
    /// **Specification:** SQL 1999
    Contains,

    /// # About
    /// **Specification:** SQL 2016
    Continue,

    /// # About
    /// **Specification:** SQL 1999
    Convert,

    /// # About
    /// **Specification:** SQL 1999
    Count,

    /// # About
    /// **Specification:** SQL 1992
    CursorName,


    /// # About
    /// **Specification:** SQL 1992
    Data,

    /// # About
    /// **Specification:** SQL 1992
    DatetimeIntervalCode,

    /// # About
    /// **Specification:** SQL 1992
    DatetimeIntervalPrecision,

    /// # About
    /// **Specification:** SQL 2003
    Defaults,

    /// # About
    /// **Specification:** SQL 2003
    Deferrable,

    /// # About
    /// **Specification:** SQL 2003
    Deferred,

    /// # About
    /// **Specification:** SQL 1999
    Defined,

    /// # About
    /// **Specification:** SQL 1999
    Definer,

    /// # About
    /// **Specification:** SQL 2003
    Degree,

    /// # About
    /// **Specification:** SQL 2003
    Depth,

    /// # About
    /// **Specification:** SQL 2003
    Derived,

    /// # About
    /// **Specification:** SQL 2003
    Desc,

    /// # About
    /// **Specification:** SQL 2016
    DescribeCatalog,

    /// # About
    /// **Specification:** SQL 2016
    DescribeName,

    /// # About
    /// **Specification:** SQL 2016
    DescribeProcedureSpecificCatalog,

    /// # About
    /// **Specification:** SQL 2016
    DescribeProcedureSpecificName,

    /// # About
    /// **Specification:** SQL 2016
    DescribeProcedureSpecificSchema,

    /// # About
    /// **Specification:** SQL 2016
    DescribeSchema,

    /// # About
    /// **Specification:** SQL 2003
    Descriptor,

    /// # About
    /// **Specification:** SQL 2003
    Diagnostics,

    /// # About
    /// **Specification:** SQL 1999
    Dispatch,

    /// # About
    /// **Specification:** SQL 2003
    Domain,

    /// # About
    /// **Specification:** SQL 1992
    DynamicFunction,

    /// # About
    /// **Specification:** SQL 1999
    DynamicFunctionCode,


    /// # About
    /// **Specification:** SQL 2016
    Encoding,

    /// # About
    /// **Specification:** SQL 2016
    Enforced,

    /// # About
    /// **Specification:** SQL 2016
    Error,

    /// # About
    /// **Specification:** SQL 2003
    Exclude,

    /// # About
    /// **Specification:** SQL 2003
    Excluding,

    /// # About
    /// **Specification:** SQL 2016
    Expression,

    /// # About
    /// **Specification:** SQL 1999
    Existing,

    /// # About
    /// **Specification:** SQL 1999
    Exists,

    /// # About
    /// **Specification:** SQL 1999
    Extract,


    /// # About
    /// **Specification:** SQL 1999
    Final,

    /// # About
    /// **Specification:** SQL 2016
    Finish,

    /// # About
    /// **Specification:** SQL 2016
    FinishCatalog,

    /// # About
    /// **Specification:** SQL 2016
    FinishName,

    /// # About
    /// **Specification:** SQL 2016
    FinishProcedureSpecificCatalog,

    /// # About
    /// **Specification:** SQL 2016
    FinishProcedureSpecificName,

    /// # About
    /// **Specification:** SQL 2016
    FinishProcedureSpecificSchema,

    /// # About
    /// **Specification:** SQL 2016
    FinishSchema,

    /// # About
    /// **Specification:** SQL 2003
    First,

    /// # About
    /// **Specification:** SQL 2016
    Flag,

    /// # About
    /// **Specification:** SQL 2003
    Following,

    /// # About
    /// **Specification:** SQL 2016
    Format,

    /// # About
    /// **Specification:** SQL 1992
    Fortran,

    /// # About
    /// **Specification:** SQL 2003
    Found,

    /// # About
    /// **Specification:** SQL 2016
    Fulfill,

    /// # About
    /// **Specification:** SQL 2016
    FulfillCatalog,

    /// # About
    /// **Specification:** SQL 2016
    FulfillName,

    /// # About
    /// **Specification:** SQL 2016
    FulfillProcedureSpecificCatalog,

    /// # About
    /// **Specification:** SQL 2016
    FulfillProcedureSpecificName,

    /// # About
    /// **Specification:** SQL 2016
    FulfillProcedureSpecificSchema,

    /// # About
    /// **Specification:** SQL 2016
    FulfillSchema,


    /// # About
    /// **Specification:** SQL 1999
    G,

    /// # About
    /// **Specification:** SQL 2003
    General,

    /// # About
    /// **Specification:** SQL 1999
    Generated,

    /// # About
    /// **Specification:** SQL 2003
    Go,

    /// # About
    /// **Specification:** SQL 2003
    Goto,

    /// # About
    /// **Specification:** SQL 1999
    Granted,


    /// # About
    /// **Specification:** SQL 2016
    HasPassThroughColumns,

    /// # About
    /// **Specification:** SQL 2016
    HasPassThruCols,

    /// # About
    /// **Specification:** SQL 1999
    Hierarchy,

    /// # About
    /// **Specification:** SQL 1999
    Hold,


    /// # About
    /// **Specification:** SQL 2016
    Ignore,

    /// # About
    /// **Specification:** SQL 2016
    Immediate,

    /// # About
    /// **Specification:** SQL 2016
    Immediately,

    /// # About
    /// **Specification:** SQL 1999
    Implementation,

    /// # About
    /// **Specification:** SQL 2003
    Including,

    /// # About
    /// **Specification:** SQL 2003
    Increment,

    /// # About
    /// **Specification:** SQL 1999
    Infix,

    /// # About
    /// **Specification:** SQL 2003
    Initially,

    /// # About
    /// **Specification:** SQL 2016
    Input,

    /// # About
    /// **Specification:** SQL 1999
    Insensitive,

    /// # About
    /// **Specification:** SQL 1999
    Instance,

    /// # About
    /// **Specification:** SQL 1999
    Instantiable,

    /// # About
    /// **Specification:** SQL 2016
    Instead,

    /// # About
    /// **Specification:** SQL 1999
    Invoker,

    /// # About
    /// **Specification:** SQL 2003
    Isolation,

    /// # About
    /// **Specification:** SQL 2016
    IsPrunable,


    /// # About
    /// **Specification:** SQL 2016
    Json,


    /// # About
    /// **Specification:** SQL 1999
    K,

    /// # About
    /// **Specification:** SQL 2016
    Keep,

    /// # About
    /// **Specification:** SQL 2003
    Key,

    /// # About
    /// **Specification:** SQL 2016
    Keys,

    /// # About
    /// **Specification:** SQL 1999
    KeyMember,

    /// # About
    /// **Specification:** SQL 1999
    KeyType,


    /// # About
    /// **Specification:** SQL 2003
    Last,

    /// # About
    /// **Specification:** SQL 1992
    Length,

    /// # About
    /// **Specification:** SQL 2003
    Level,

    /// # About
    /// **Specification:** SQL 2003
    Locator,

    /// # About
    /// **Specification:** SQL 1999
    Lower,


    /// # About
    /// **Specification:** SQL 1999
    M,

    /// # About
    /// **Specification:** SQL 2003
    Map,

    /// # About
    /// **Specification:** SQL 2003
    Matched,

    /// # About
    /// **Specification:** SQL 1999
    Max,

    /// # About
    /// **Specification:** SQL 2016
    Maxvalue,

    /// # About
    /// **Specification:** SQL 1992
    MessageLength,

    /// # About
    /// **Specification:** SQL 1992
    MessageOctetLength,

    /// # About
    /// **Specification:** SQL 1992
    MessageText,

    /// # About
    /// **Specification:** SQL 1999
    Method,

    /// # About
    /// **Specification:** SQL 1999
    Min,

    /// # About
    /// **Specification:** SQL 2003
    Minvalue,

    /// # About
    /// **Specification:** SQL 1999
    Mod,

    /// # About
    /// **Specification:** SQL 1992
    More,

    /// # About
    /// **Specification:** SQL 1992
    Mumps,


    /// # About
    /// **Specification:** SQL 1992
    Name,

    /// # About
    /// **Specification:** SQL 2003
    Names,

    /// # About
    /// **Specification:** SQL 1992
    Nullable,

    /// # About
    /// **Specification:** SQL 1999
    Nullif,

    /// # About
    /// **Specification:** SQL 2003
    Nulls,

    /// # About
    /// **Specification:** SQL 1992
    Number,


    /// # About
    /// **Specification:** SQL 2003
    Object,

    /// # About
    /// **Specification:** SQL 2003
    Octets,

    /// # About
    /// **Specification:** SQL 1999
    OctetLength,

    /// # About
    /// **Specification:** SQL 2003
    Option,

    /// # About
    /// **Specification:** SQL 1999
    Options,

    /// # About
    /// **Specification:** SQL 2003
    Ordering,

    /// # About
    /// **Specification:** SQL 2003
    Ordinality,

    /// # About
    /// **Specification:** SQL 2003
    Others,

    /// # About
    /// **Specification:** SQL 2016
    Output,

    /// # About
    /// **Specification:** SQL 2016
    Overflow,

    /// # About
    /// **Specification:** SQL 1999
    Overlaps,

    /// # About
    /// **Specification:** SQL 1999
    Overlay,

    /// # About
    /// **Specification:** SQL 1999
    Overriding,


    /// # About
    /// **Specification:** SQL 2016
    P,

    /// # About
    /// **Specification:** SQL 2003
    Pad,

    /// # About
    /// **Specification:** SQL 1999
    ParameterMode,

    /// # About
    /// **Specification:** SQL 1999
    ParameterName,

    /// # About
    /// **Specification:** SQL 1999
    ParameterOrdinalPosition,

    /// # About
    /// **Specification:** SQL 1999
    ParameterSpecificCatalog,

    /// # About
    /// **Specification:** SQL 1999
    ParameterSpecificName,

    /// # About
    /// **Specification:** SQL 1999
    ParameterSpecificSchema,

    /// # About
    /// **Specification:** SQL 2003
    Partial,

    /// # About
    /// **Specification:** SQL 1992
    Pascal,

    /// # About
    /// **Specification:** SQL 2016
    Pass,

    /// # About
    /// **Specification:** SQL 2016
    Passing,

    /// # About
    /// **Specification:** SQL 2016
    Past,

    /// # About
    /// **Specification:** SQL 2003
    Path,

    /// # About
    /// **Specification:** SQL 2016
    Placing,

    /// # About
    /// **Specification:** SQL 2016
    Plan,

    /// # About
    /// **Specification:** SQL 1992
    Pli,

    /// # About
    /// **Specification:** SQL 1999
    Position,

    /// # About
    /// **Specification:** SQL 2003
    Preceding,

    /// # About
    /// **Specification:** SQL 2003
    Preserve,

    /// # About
    /// **Specification:** SQL 2003
    Prior,

    /// # About
    /// **Specification:** SQL 2016
    Private,

    /// # About
    /// **Specification:** SQL 2016
    PrivateParameters,

    /// # About
    /// **Specification:** SQL 2016
    PrivateParamsS,

    /// # About
    /// **Specification:** SQL 2003
    Privileges,

    /// # About
    /// **Specification:** SQL 2016
    Prune,

    /// # About
    /// **Specification:** SQL 2003
    Public,


    /// # About
    /// **Specification:** SQL 2016
    Quotes,


    /// # About
    /// **Specification:** SQL 2003
    Read,

    /// # About
    /// **Specification:** SQL 2003
    Relative,

    /// # About
    /// **Specification:** SQL 1992
    Repeatable,

    /// # About
    /// **Specification:** SQL 2016
    Respect,

    /// # About
    /// **Specification:** SQL 2003
    Restart,

    /// # About
    /// **Specification:** SQL 2016
    Restrict,

    /// # About
    /// **Specification:** SQL 2003
    ReturnedCardinality,

    /// # About
    /// **Specification:** SQL 1992
    ReturnedLength,

    /// # About
    /// **Specification:** SQL 1992
    ReturnedOctetLength,

    /// # About
    /// **Specification:** SQL 1992
    ReturnedSqlstate,

    /// # About
    /// **Specification:** SQL 2016
    Returning,

    /// # About
    /// **Specification:** SQL 2016
    ReturnsOnlyPassThrough,

    /// # About
    /// **Specification:** SQL 2016
    RetOnlyPassThru,

    /// # About
    /// **Specification:** SQL 2003
    Role,

    /// # About
    /// **Specification:** SQL 2003
    Routine,

    /// # About
    /// **Specification:** SQL 1999
    RoutineCatalog,

    /// # About
    /// **Specification:** SQL 1999
    RoutineName,

    /// # About
    /// **Specification:** SQL 1999
    RoutineSchema,

    /// # 1992
    /// **Specification:** SQL 1999
    RowCount,


    /// # About
    /// **Specification:** SQL 2016
    Scalar,

    /// # About
    /// **Specification:** SQL 1992
    Scale,

    /// # About
    /// **Specification:** SQL 2003
    Schema,

    /// # About
    /// **Specification:** SQL 1992
    SchemaName,

    /// # About
    /// **Specification:** SQL 2003
    ScopeCatalog,

    /// # About
    /// **Specification:** SQL 2003
    ScopeName,

    /// # About
    /// **Specification:** SQL 2003
    ScopeSchema,

    /// # About
    /// **Specification:** SQL 2003
    Section,

    /// # About
    /// **Specification:** SQL 1999
    Security,

    /// Reserved identifier in `Rust` ^_^
    /// # About
    /// **Specification:** SQL 1999
    #[strum(to_string="SELF")]
    Self_,

    /// # About
    /// **Specification:** SQL 1999
    Sensitive,

    /// # About
    /// **Specification:** SQL 2003
    Sequence,

    /// # About
    /// **Specification:** SQL 1992
    Serializable,

    /// # About
    /// **Specification:** SQL 1992
    ServerName,

    /// # About
    /// **Specification:** SQL 2003
    Session,

    /// # About
    /// **Specification:** SQL 2003
    Sets,

    /// # About
    /// **Specification:** SQL 1999
    Simple,

    /// # About
    /// **Specification:** SQL 2003
    Size,

    /// # About
    /// **Specification:** SQL 1999
    Source,

    /// # About
    /// **Specification:** SQL 2003
    Space,

    /// # About
    /// **Specification:** SQL 1999
    SpecificName,

    /// # About
    /// **Specification:** SQL 1999
    Similar,

    /// # About
    /// **Specification:** SQL 2016
    StartCatalog,

    /// # About
    /// **Specification:** SQL 2016
    StartName,

    /// # About
    /// **Specification:** SQL 2016
    StartProcedureSpecificCatalog,

    /// # About
    /// **Specification:** SQL 2016
    StartProcedureSpecificName,

    /// # About
    /// **Specification:** SQL 2016
    StartProcedureSpecificSchema,

    /// # About
    /// **Specification:** SQL 2016
    StartSchema,

    /// # About
    /// **Specification:** SQL 2003
    State,

    /// # About
    /// **Specification:** SQL 2003
    Statement,

    /// # About
    /// **Specification:** SQL 2016
    String,

    /// # About
    /// **Specification:** SQL 2003
    Structure,

    /// # About
    /// **Specification:** SQL 1999
    Style,

    /// # About
    /// **Specification:** SQL 1992
    SubclassOrigin,

    /// # About
    /// **Specification:** SQL 1999
    Sublist,

    /// # About
    /// **Specification:** SQL 1999
    Substring,

    /// # About
    /// **Specification:** SQL 1999
    Sum,

    /// # About
    /// **Specification:** SQL 1999
    Symmetric,

    /// # About
    /// **Specification:** SQL 1999
    System,


    /// # About
    /// **Specification:** SQL 2016
    T,

    /// # About
    /// **Specification:** SQL 1992
    TableName,

    /// # About
    /// **Specification:** SQL 2016
    TableSemantics,

    /// # About
    /// **Specification:** SQL 2003
    Temporary,

    /// # About
    /// **Specification:** SQL 2016
    Through,

    /// # About
    /// **Specification:** SQL 2003
    Ties,

    /// # About
    /// **Specification:** SQL 2003
    TopLevelCount,

    /// # About
    /// **Specification:** SQL 2003
    Transaction,

    /// # About
    /// **Specification:** SQL 1999
    TransactionActive,

    /// # About
    /// **Specification:** SQL 1999
    TransactionsCommitted,

    /// # About
    /// **Specification:** SQL 1999
    TransactionsRolledBack,

    /// # About
    /// **Specification:** SQL 1999
    Transform,

    /// # About
    /// **Specification:** SQL 1999
    Transforms,

    /// # About
    /// **Specification:** SQL 1999
    Translate,

    /// # About
    /// **Specification:** SQL 1999
    TriggerCatalog,

    /// # About
    /// **Specification:** SQL 1999
    TriggerSchema,

    /// # About
    /// **Specification:** SQL 1999
    TriggerName,

    /// # About
    /// **Specification:** SQL 1999
    Trim,

    /// # About
    /// **Specification:** SQL 1992
    Type,


    /// # About
    /// **Specification:** SQL 2003
    Unbounded,

    /// # About
    /// **Specification:** SQL 1992
    Uncommitted,

    /// # About
    /// **Specification:** SQL 2016
    Unconditional,

    /// # About
    /// **Specification:** SQL 2003
    Under,

    /// # About
    /// **Specification:** SQL 1992
    Unnamed,

    /// # About
    /// **Specification:** SQL 1999
    Upper,

    /// # About
    /// **Specification:** SQL 2003
    Usage,

    /// # About
    /// **Specification:** SQL 1999
    UserDefinedTypeCatalog,

    /// # About
    /// **Specification:** SQL 2003
    UserDefinedTypeCode,

    /// # About
    /// **Specification:** SQL 1999
    UserDefinedTypeName,

    /// # About
    /// **Specification:** SQL 1999
    UserDefinedTypeSchema,

    /// # About
    /// **Specification:** SQL 2016
    UTF16,

    /// # About
    /// **Specification:** SQL 2016
    UTF32,

    /// # About
    /// **Specification:** SQL 2016
    UTF8,


    /// # About
    /// **Specification:** SQL 2003
    View,


    /// # About
    /// **Specification:** SQL 2003
    Work,

    /// # About
    /// **Specification:** SQL 2016
    Wrapper,

    /// # About
    /// **Specification:** SQL 2003
    Write,


    /// # About
    /// **Specification:** SQL 2003
    Zone,
}
