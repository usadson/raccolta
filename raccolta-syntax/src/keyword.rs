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
/// **Specification:** SQL 1992, SQL 1999, SQL 2003, SQL 2016
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter, strum::Display)]
#[non_exhaustive]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum NonReservedWord {
    /// # About
    /// **Specification:** SQL 2003
    A,

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
    /// **Specification:** SQL 1986
    Asc,

    /// # About
    /// **Specification:** SQL 2003
    Assertion,

    /// # About
    /// **Specification:** SQL 1999
    Assignment,

    /// # About
    /// **Specification:** SQL 2003
    Attribute,

    /// # About
    /// **Specification:** SQL 2003
    Attributes,


    /// # About
    /// **Specification:** SQL 2003
    Before,

    /// # About
    /// **Specification:** SQL 2003
    Bernoulli,

    /// # About
    /// **Specification:** SQL 2003
    Breadth,


    /// # About
    /// **Specification:** SQL 1992
    C,

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
    /// **Specification:** SQL 1992
    ClassOrigin,

    /// # About
    /// **Specification:** SQL 1986
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
    /// **Specification:** SQL 1986
    Continue,

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
    /// **Specification:** SQL 1986
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
    /// **Specification:** SQL 1986
    Fortran,

    /// # About
    /// **Specification:** SQL 1986
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
    /// **Specification:** SQL 1986
    Go,

    /// # About
    /// **Specification:** SQL 1986
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
    /// **Specification:** SQL 2003
    Initially,

    /// # About
    /// **Specification:** SQL 2016
    Input,

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
    M,

    /// # About
    /// **Specification:** SQL 2003
    Map,

    /// # About
    /// **Specification:** SQL 2003
    Matched,

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
    /// **Specification:** SQL 2003
    Minvalue,

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
    /// **Specification:** SQL 2016
    Nested,

    /// # About
    /// **Specification:** SQL 2016
    Nesting,

    /// # About
    /// **Specification:** SQL 2016
    Next,

    /// # About
    /// **Specification:** SQL 2016
    Nfc,

    /// # About
    /// **Specification:** SQL 2016
    Nfd,

    /// # About
    /// **Specification:** SQL 2016
    Nfkc,

    /// # About
    /// **Specification:** SQL 2016
    Nfkd,

    /// # About
    /// **Specification:** SQL 2016
    Normalized,

    /// # About
    /// **Specification:** SQL 1992
    Nullable,

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
    /// **Specification:** SQL 1986
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
    /// **Specification:** SQL 1986
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
    /// **Specification:** SQL 1986
    Pli,

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
    /// **Specification:** SQL 1986
    Privileges,

    /// # About
    /// **Specification:** SQL 2016
    Prune,

    /// # About
    /// **Specification:** SQL 1986
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
    /// **Specification:** SQL 1986
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
    /// **Specification:** SQL 1986
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
    TriggerCatalog,

    /// # About
    /// **Specification:** SQL 1999
    TriggerSchema,

    /// # About
    /// **Specification:** SQL 1999
    TriggerName,

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
    /// **Specification:** SQL 1986
    View,


    /// # About
    /// **Specification:** SQL 1986
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

/// Reserved words (`<reserved word>`) are words that are only allowed in
/// context with semantic meaning. If you'd want to use these as column,
/// database, schema, table, or view names, you have to escape the identifiers.
///
/// SQL-86 didn't have these `<reserved words>`, as they were introduced in
/// SQL-1992. In SQL-86, they were simply known as `<key words>`.
///
/// # About
/// **Specification:** SQL 86, SQL 1992, SQL 1999, SQL 2003, SQL 2016
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(AsRefStr, EnumIter, strum::Display)]
#[non_exhaustive]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum ReservedWord {
    /// # About
    /// **Specification:** SQL 1999
    Abs,

    /// # About
    /// **Specification:** SQL 2016
    Acos,

    /// # About
    /// **Specification:** SQL 1986
    All,

    /// # About
    /// **Specification:** SQL 2016
    Allocate,

    /// # About
    /// **Specification:** SQL 2016
    Alter,

    /// # About
    /// **Specification:** SQL 1986
    And,

    /// # About
    /// **Specification:** SQL 1986
    Any,

    /// # About
    /// **Specification:** SQL 2016
    Are,

    /// # About
    /// **Specification:** SQL 2016
    Array,

    /// # About
    /// **Specification:** SQL 2016
    ArrayAgg,

    /// # About
    /// **Specification:** SQL 2016
    ArrayMaxCardinality,

    /// # About
    /// **Specification:** SQL 1986
    As,

    /// # About
    /// **Specification:** SQL 1999
    Asensitive,

    /// # About
    /// **Specification:** SQL 2016
    Asin,

    /// # About
    /// **Specification:** SQL 1999
    Asymmetric,

    /// # About
    /// **Specification:** SQL 2016
    At,

    /// # About
    /// **Specification:** SQL 2016
    Atan,

    /// # About
    /// **Specification:** SQL 1999
    Atomic,

    /// # About
    /// **Specification:** SQL 1986
    Authorization,

    /// # About
    /// **Specification:** SQL 1986
    Avg,


    /// # About
    /// **Specification:** SQL 1986
    Begin,

    /// # About
    /// **Specification:** SQL 2016
    BeginFrame,

    /// # About
    /// **Specification:** SQL 2016
    BeginPartition,

    /// # About
    /// **Specification:** SQL 1986
    Between,

    /// # About
    /// **Specification:** SQL 2016
    Bigint,

    /// # About
    /// **Specification:** SQL 2016
    Binary,

    /*
    /// # About
    /// **Specification:** SQL 1999
    Bitvar,
    /// # About
    /// **Specification:** SQL 1999
    BitLength,
    */

    /// # About
    /// **Specification:** SQL 2016
    Blob,

    /// # About
    /// **Specification:** SQL 2016
    Boolean,

    /// # About
    /// **Specification:** SQL 2016
    Both,

    /// # About
    /// **Specification:** SQL 1986
    By,


    /// # About
    /// **Specification:** SQL 2016
    Call,

    /// # About
    /// **Specification:** SQL 1999
    Called,

    /// # About
    /// **Specification:** SQL 1999
    Cardinality,

    /// # About
    /// **Specification:** SQL 2016
    Cascaded,

    /// # About
    /// **Specification:** SQL 2016
    Case,

    /// # About
    /// **Specification:** SQL 2016
    Cast,

    /// # About
    /// **Specification:** SQL 2016
    Ceil,

    /// # About
    /// **Specification:** SQL 2016
    Ceiling,

    /// # About
    /// **Specification:** SQL 1999
    CharLength,

    /// # About
    /// **Specification:** SQL 1986
    Char,

    /// # About
    /// **Specification:** SQL 1986
    Character,

    /// # About
    /// **Specification:** SQL 1999
    CharacterLength,

    /// # About
    /// **Specification:** SQL 1986
    Check,

/*
    /// # About
    /// **Specification:** SQL 1999
    Checked,
*/

    /// # About
    /// **Specification:** SQL 2016
    Classifier,

    /// # About
    /// **Specification:** SQL 2016
    Clob,

    /// # About
    /// **Specification:** SQL 1986
    Close,

    /// # About
    /// **Specification:** SQL 1999
    Coalesce,

    /// # About
    /// **Specification:** SQL 2016
    Collate,

    /// # About
    /// **Specification:** SQL 2016
    Collect,

    /// # About
    /// **Specification:** SQL 2016
    Column,

    /// # About
    /// **Specification:** SQL 1986
    Commit,

    /// # About
    /// **Specification:** SQL 2016
    Condition,

    /// # About
    /// **Specification:** SQL 2016
    Connect,

    /// # About
    /// **Specification:** SQL 2016
    Constraint,

    /// # About
    /// **Specification:** SQL 1999
    Contains,

    /// # About
    /// **Specification:** SQL 1999
    Convert,

    /// # About
    /// **Specification:** SQL 2016
    Copy,

    /// # About
    /// **Specification:** SQL 2016
    Corr,

    /// # About
    /// **Specification:** SQL 2016
    Corresponding,

    /// # About
    /// **Specification:** SQL 2016
    Cos,

    /// # About
    /// **Specification:** SQL 2016
    Cosh,

    /// # About
    /// **Specification:** SQL 1986
    Count,

    /// # About
    /// **Specification:** SQL 2016
    CovarPop,

    /// # About
    /// **Specification:** SQL 2016
    CovarSamp,

    /// # About
    /// **Specification:** SQL 1986
    Create,

    /// # About
    /// **Specification:** SQL 2016
    Cross,

    /// # About
    /// **Specification:** SQL 2016
    Cube,

    /// # About
    /// **Specification:** SQL 2016
    CumeDist,

    /// # About
    /// **Specification:** SQL 1986
    Current,

    /// # About
    /// **Specification:** SQL 2016
    CurrentCatalog,

    /// # About
    /// **Specification:** SQL 2016
    CurrentDate,

    /// # About
    /// **Specification:** SQL 2016
    CurrentDefaultTransformGroup,

    /// # About
    /// **Specification:** SQL 2016
    CurrentPath,

    /// # About
    /// **Specification:** SQL 2016
    CurrentRole,

    /// # About
    /// **Specification:** SQL 2016
    CurrentRow,

    /// # About
    /// **Specification:** SQL 2016
    CurrentSchema,

    /// # About
    /// **Specification:** SQL 2016
    CurrentTime,

    /// # About
    /// **Specification:** SQL 2016
    CurrentTimestamp,

    /// # About
    /// **Specification:** SQL 2016
    CurrentTransformGroupForType,

    /// # About
    /// **Specification:** SQL 2016
    CurrentUser,

    /// # About
    /// **Specification:** SQL 1986
    Cursor,

    /// # About
    /// **Specification:** SQL 2016
    Cycle,


    /// # About
    /// **Specification:** SQL 2016
    Date,

    /// # About
    /// **Specification:** SQL 2016
    Day,

    /// # About
    /// **Specification:** SQL 2016
    Deallocate,

    /// # About
    /// **Specification:** SQL 1986
    Dec,

    /// # About
    /// **Specification:** SQL 1986
    Decimal,

    /// # About
    /// **Specification:** SQL 2016
    Decfloat,

    /// # About
    /// **Specification:** SQL 1986
    Declare,

    /// # About
    /// **Specification:** SQL 2016
    Default,

    /// # About
    /// **Specification:** SQL 2016
    Define,

    /// # About
    /// **Specification:** SQL 1986
    Delete,

    /// # About
    /// **Specification:** SQL 2016
    DenseRank,

    /// # About
    /// **Specification:** SQL 2016
    Deref,

    /// # About
    /// **Specification:** SQL 2016
    Describe,

    /// # About
    /// **Specification:** SQL 2016
    Deterministic,

    /// # About
    /// **Specification:** SQL 2016
    Disconnect,

    /// # About
    /// **Specification:** SQL 1986
    Distinct,

    /// # About
    /// **Specification:** SQL 1986
    Double,

    /// # About
    /// **Specification:** SQL 2016
    Drop,

    /// # About
    /// **Specification:** SQL 2016
    Dynamic,


    /// # About
    /// **Specification:** SQL 2016
    Each,

    /// # About
    /// **Specification:** SQL 2016
    Element,

    /// # About
    /// **Specification:** SQL 2016
    Else,

    /// # About
    /// **Specification:** SQL 2016
    Empty,

    /// # About
    /// **Specification:** SQL 1986
    End,

    /// # About
    /// **Specification:** SQL 2016
    EndFrame,

    /// # About
    /// **Specification:** SQL 2016
    EndPartition,

    /// # About
    /// **Specification:** SQL 2016
    #[strum(to_string="END-EXEC")]
    EndExec,

    /// # About
    /// **Specification:** SQL 2016
    Equals,

    /// # About
    /// **Specification:** SQL 1986
    Escape,

    /// # About
    /// **Specification:** SQL 2016
    Every,

    /// # About
    /// **Specification:** SQL 2016
    Except,

    /// # About
    /// **Specification:** SQL 1986
    Exec,

    /// # About
    /// **Specification:** SQL 2016
    Execute,

    /*
    /// # About
    /// **Specification:** SQL 1999
    Existing,
    */

    /// # About
    /// **Specification:** SQL 1986
    Exists,

    /// # About
    /// **Specification:** SQL 2016
    Exp,

    /// # About
    /// **Specification:** SQL 2016
    External,

    /// # About
    /// **Specification:** SQL 1999
    Extract,


    /// # About
    /// **Specification:** SQL 2016
    False,

    /// # About
    /// **Specification:** SQL 1986
    Fetch,

    /// # About
    /// **Specification:** SQL 2016
    Filter,

    /// # About
    /// **Specification:** SQL 2016
    FirstValue,

    /// # About
    /// **Specification:** SQL 1986
    Float,

    /// # About
    /// **Specification:** SQL 2016
    Floor,

    /// # About
    /// **Specification:** SQL 1986
    For,

    /// # About
    /// **Specification:** SQL 2016
    Foreign,

    /// # About
    /// **Specification:** SQL 2016
    FrameRow,

    /// # About
    /// **Specification:** SQL 2016
    Free,

    /// # About
    /// **Specification:** SQL 1986
    From,

    /// # About
    /// **Specification:** SQL 2016
    Full,

    /// # About
    /// **Specification:** SQL 2016
    Function,

    /// # About
    /// **Specification:** SQL 2016
    Fusion,


    /// # About
    /// **Specification:** SQL 2016
    Get,

    /// # About
    /// **Specification:** SQL 2016
    Global,

    /// # About
    /// **Specification:** SQL 1986
    Grant,

    /// # About
    /// **Specification:** SQL 1986
    Group,

    /// # About
    /// **Specification:** SQL 2016
    Grouping,

    /// # About
    /// **Specification:** SQL 2016
    Groups,


    /// # About
    /// **Specification:** SQL 1986
    Having,

    /// # About
    /// **Specification:** SQL 1999
    Hold,

    /// # About
    /// **Specification:** SQL 2016
    Hour,


    /// # About
    /// **Specification:** SQL 2016
    Identity,

    /// # About
    /// **Specification:** SQL 1986
    In,

    /// # About
    /// **Specification:** SQL 1986
    Indicator,

    /// # About
    /// **Specification:** SQL 1999
    Infix,

    /// # About
    /// **Specification:** SQL 2016
    Initial,

    /// # About
    /// **Specification:** SQL 2016
    Inner,

    /// # About
    /// **Specification:** SQL 2016
    Inout,

    /// # About
    /// **Specification:** SQL 1999
    Insensitive,

    /// # About
    /// **Specification:** SQL 1986
    Insert,

    /// # About
    /// **Specification:** SQL 1986
    Int,

    /// # About
    /// **Specification:** SQL 1986
    Integer,

    /// # About
    /// **Specification:** SQL 2016
    Intersect,

    /// # About
    /// **Specification:** SQL 2016
    Intersection,

    /// # About
    /// **Specification:** SQL 2016
    Interval,

    /// # About
    /// **Specification:** SQL 1986
    Into,

    /// # About
    /// **Specification:** SQL 1986
    Is,


    /// # About
    /// **Specification:** SQL 2016
    Join,

    /// # About
    /// **Specification:** SQL 2016
    JsonArray,

    /// # About
    /// **Specification:** SQL 2016
    JsonArrayagg,

    /// # About
    /// **Specification:** SQL 2016
    JsonExists,

    /// # About
    /// **Specification:** SQL 2016
    JsonObject,

    /// # About
    /// **Specification:** SQL 2016
    JsonObjectagg,

    /// # About
    /// **Specification:** SQL 2016
    JsonQuery,

    /// # About
    /// **Specification:** SQL 2016
    JsonTable,

    /// # About
    /// **Specification:** SQL 2016
    JsonTablePrimitive,

    /// # About
    /// **Specification:** SQL 2016
    JsonValue,


    /// # About
    /// **Specification:** SQL 2016
    Lag,

    /// # About
    /// **Specification:** SQL 1986
    Language,

    /// # About
    /// **Specification:** SQL 2016
    Large,

    /// # About
    /// **Specification:** SQL 2016
    LastValue,

    /// # About
    /// **Specification:** SQL 2016
    Lateral,

    /// # About
    /// **Specification:** SQL 2016
    Lead,

    /// # About
    /// **Specification:** SQL 2016
    Leading,

    /// # About
    /// **Specification:** SQL 2016
    Left,

    /// # About
    /// **Specification:** SQL 1986
    Like,

    /// # About
    /// **Specification:** SQL 2016
    LikeRegex,

    /// # About
    /// **Specification:** SQL 2016
    Listagg,

    /// # About
    /// **Specification:** SQL 2016
    Ln,

    /// # About
    /// **Specification:** SQL 2016
    Local,

    /// # About
    /// **Specification:** SQL 2016
    Localtime,

    /// # About
    /// **Specification:** SQL 2016
    Localtimestamp,

    /// # About
    /// **Specification:** SQL 2016
    Log,

    /// # About
    /// **Specification:** SQL 2016
    Log10,

    /// # About
    /// **Specification:** SQL 1999
    Lower,


    /// # About
    /// **Specification:** SQL 2016
    Match,

    /// # About
    /// **Specification:** SQL 2016
    MatchNumber,

    /// # About
    /// **Specification:** SQL 2016
    MatchRecognize,

    /// # About
    /// **Specification:** SQL 2016
    Matches,

    /// # About
    /// **Specification:** SQL 1986
    Max,

    /// # About
    /// **Specification:** SQL 2016
    Member,

    /// # About
    /// **Specification:** SQL 2016
    Merge,

    /// # About
    /// **Specification:** SQL 1999
    Method,

    /// # About
    /// **Specification:** SQL 1986
    Min,

    /// # About
    /// **Specification:** SQL 2016
    Minute,

    /// # About
    /// **Specification:** SQL 1999
    Mod,

    /// # About
    /// **Specification:** SQL 2016
    Modifies,

    /// # About
    /// **Specification:** SQL 1986
    Module,

    /// # About
    /// **Specification:** SQL 2016
    Month,

    /// # About
    /// **Specification:** SQL 2016
    Multiset,


    /// # About
    /// **Specification:** SQL 2016
    National,

    /// # About
    /// **Specification:** SQL 2016
    Natural,

    /// # About
    /// **Specification:** SQL 2016
    Nchar,

    /// # About
    /// **Specification:** SQL 2016
    Nclob,

    /// # About
    /// **Specification:** SQL 2016
    New,

    /// # About
    /// **Specification:** SQL 2016
    No,

    /// # About
    /// **Specification:** SQL 2016
    None,

    /// # About
    /// **Specification:** SQL 2016
    Normalize,

    /// # About
    /// **Specification:** SQL 1986
    Not,

    /// # About
    /// **Specification:** SQL 2016
    NthValue,

    /// # About
    /// **Specification:** SQL 2016
    Ntile,

    /// # About
    /// **Specification:** SQL 1986
    Null,

    /// # About
    /// **Specification:** SQL 1999
    Nullif,

    /// # About
    /// **Specification:** SQL 1986
    Numeric,


    /// # About
    /// **Specification:** SQL 1999
    OctetLength,

    /// # About
    /// **Specification:** SQL 2016
    OccurrencesRegex,

    /// # About
    /// **Specification:** SQL 1986
    Of,

    /// # About
    /// **Specification:** SQL 2016
    Offset,

    /// # About
    /// **Specification:** SQL 2016
    Old,

    /// # About
    /// **Specification:** SQL 2016
    Omit,

    /// # About
    /// **Specification:** SQL 1986
    On,

    /// # About
    /// **Specification:** SQL 2016
    One,

    /// # About
    /// **Specification:** SQL 2016
    Only,

    /// # About
    /// **Specification:** SQL 1986
    Open,

    /// # About
    /// **Specification:** SQL 1986
    Or,

    /// # About
    /// **Specification:** SQL 1986
    Order,

    /// # About
    /// **Specification:** SQL 2016
    Out,

    /// # About
    /// **Specification:** SQL 2016
    Outer,

    /// # About
    /// **Specification:** SQL 2016
    Over,

    /// # About
    /// **Specification:** SQL 1999
    Overlaps,

    /// # About
    /// **Specification:** SQL 1999
    Overlay,


    /// # About
    /// **Specification:** SQL 2016
    Parameter,

    /// # About
    /// **Specification:** SQL 2016
    Partition,

    /// # About
    /// **Specification:** SQL 2016
    Pattern,

    /// # About
    /// **Specification:** SQL 2016
    Per,

    /// # About
    /// **Specification:** SQL 2016
    Percent,

    /// # About
    /// **Specification:** SQL 2016
    PercentRank,

    /// # About
    /// **Specification:** SQL 2016
    PercentileCont,

    /// # About
    /// **Specification:** SQL 2016
    PercentileDisc,

    /// # About
    /// **Specification:** SQL 2016
    Period,

    /// # About
    /// **Specification:** SQL 2016
    Portion,

    /// # About
    /// **Specification:** SQL 1999
    Position,

    /// # About
    /// **Specification:** SQL 2016
    PositionRegex,

    /// # About
    /// **Specification:** SQL 2016
    Power,

    /// # About
    /// **Specification:** SQL 2016
    Precedes,

    /// # About
    /// **Specification:** SQL 1986
    Precision,

    /// # About
    /// **Specification:** SQL 2016
    Prepare,

    /// # About
    /// **Specification:** SQL 2016
    Primary,

    /// # About
    /// **Specification:** SQL 1986
    Procedure,

    /// # About
    /// **Specification:** SQL 2016
    Ptf,


    /// # About
    /// **Specification:** SQL 2016
    Range,

    /// # About
    /// **Specification:** SQL 2016
    Rank,

    /// # About
    /// **Specification:** SQL 2016
    Reads,

    /// # About
    /// **Specification:** SQL 1986
    Real,

    /// # About
    /// **Specification:** SQL 2016
    Recursive,

    /// # About
    /// **Specification:** SQL 2016
    Ref,

    /// # About
    /// **Specification:** SQL 2016
    References,

    /// # About
    /// **Specification:** SQL 2016
    Referencing,

    /// # About
    /// **Specification:** SQL 2016
    RegrAvgx,

    /// # About
    /// **Specification:** SQL 2016
    RegrAvgy,

    /// # About
    /// **Specification:** SQL 2016
    RegrCount,

    /// # About
    /// **Specification:** SQL 2016
    RegrIntercept,

    /// # About
    /// **Specification:** SQL 2016
    RegrR2,

    /// # About
    /// **Specification:** SQL 2016
    RegrSlope,

    /// # About
    /// **Specification:** SQL 2016
    RegrSxx,

    /// # About
    /// **Specification:** SQL 2016
    RegrSxy,

    /// # About
    /// **Specification:** SQL 2016
    RegrSyy,

    /// # About
    /// **Specification:** SQL 2016
    Release,

    /// # About
    /// **Specification:** SQL 2016
    Result,

    /// # About
    /// **Specification:** SQL 2016
    Return,

    /// # About
    /// **Specification:** SQL 2016
    Returns,

    /// # About
    /// **Specification:** SQL 2016
    Revoke,

    /// # About
    /// **Specification:** SQL 2016
    Right,

    /// # About
    /// **Specification:** SQL 1986
    Rollback,

    /// # About
    /// **Specification:** SQL 2016
    Rollup,

    /// # About
    /// **Specification:** SQL 2016
    Row,

    /// # About
    /// **Specification:** SQL 2016
    RowNumber,

    /// # About
    /// **Specification:** SQL 2016
    Rows,

    /// # About
    /// **Specification:** SQL 2016
    Running,


    /// # About
    /// **Specification:** SQL 2016
    Savepoint,

    /// # About
    /// **Specification:** SQL 2016
    Scope,

    /// # About
    /// **Specification:** SQL 2016
    Scroll,

    /// # About
    /// **Specification:** SQL 2016
    Search,

    /// # About
    /// **Specification:** SQL 2016
    Second,

    /// # About
    /// **Specification:** SQL 2016
    Seek,

    /// # About
    /// **Specification:** SQL 1986
    Select,

    /// # About
    /// **Specification:** SQL 1999
    Sensitive,

    /// # About
    /// **Specification:** SQL 2016
    SessionUser,

    /// # About
    /// **Specification:** SQL 1986
    Set,

    /// # About
    /// **Specification:** SQL 2016
    Show,

    /// # About
    /// **Specification:** SQL 1999
    Similar,

    /// # About
    /// **Specification:** SQL 2016
    Sin,

    /// # About
    /// **Specification:** SQL 2016
    Sinh,

    /// # About
    /// **Specification:** SQL 2016
    Skip,

    /// # About
    /// **Specification:** SQL 1986
    Smallint,

    /// # About
    /// **Specification:** SQL 1986
    Some,

    /// # About
    /// **Specification:** SQL 2016
    Specific,

    /// # About
    /// **Specification:** SQL 2016
    Specifictype,

    /// # About
    /// **Specification:** SQL 1986
    Sql,

    /// # About
    /// **Specification:** SQL 1986
    Sqlcode,

    /// # About
    /// **Specification:** SQL 2016
    Sqlexception,

    /// # About
    /// **Specification:** SQL 1986
    Sqlerror,

    /// # About
    /// **Specification:** SQL 2016
    Sqlstate,

    /// # About
    /// **Specification:** SQL 2016
    Sqlwarning,

    /// # About
    /// **Specification:** SQL 2016
    Sqrt,

    /// # About
    /// **Specification:** SQL 2016
    Start,

    /// # About
    /// **Specification:** SQL 2016
    Static,

    /// # About
    /// **Specification:** SQL 2016
    StddevPop,

    /// # About
    /// **Specification:** SQL 2016
    StddevSamp,

    /// # About
    /// **Specification:** SQL 2016
    Submultiset,

    /// # About
    /// **Specification:** SQL 2016
    Sublist,

    /// # About
    /// **Specification:** SQL 1986
    Subset,

    /// # About
    /// **Specification:** SQL 1999
    Substring,

    /// # About
    /// **Specification:** SQL 2016
    SubstringRegex,

    /// # About
    /// **Specification:** SQL 2016
    Succeeds,

    /// # About
    /// **Specification:** SQL 1986
    Sum,

    /// # About
    /// **Specification:** SQL 1999
    Symmetric,

    /// # About
    /// **Specification:** SQL 1999
    System,

    /// # About
    /// **Specification:** SQL 2016
    SystemTime,

    /// # About
    /// **Specification:** SQL 2016
    SystemUser,


    /// # About
    /// **Specification:** SQL 1986
    Table,

    /// # About
    /// **Specification:** SQL 2016
    Tablesample,

    /// # About
    /// **Specification:** SQL 2016
    Tan,

    /// # About
    /// **Specification:** SQL 2016
    Tanh,

    /// # About
    /// **Specification:** SQL 2016
    Then,

    /// # About
    /// **Specification:** SQL 2016
    Time,

    /// # About
    /// **Specification:** SQL 2016
    Timestamp,

    /// # About
    /// **Specification:** SQL 2016
    TimezoneHour,

    /// # About
    /// **Specification:** SQL 2016
    TimezoneMinute,

    /// # About
    /// **Specification:** SQL 1986
    To,

    /// # About
    /// **Specification:** SQL 2016
    Trailing,

    /// # About
    /// **Specification:** SQL 1999
    Translate,

    /// # About
    /// **Specification:** SQL 2016
    TranslateRegex,

    /// # About
    /// **Specification:** SQL 2016
    Translation,

    /// # About
    /// **Specification:** SQL 2016
    Treat,

    /// # About
    /// **Specification:** SQL 2016
    Trigger,

    /// # About
    /// **Specification:** SQL 1999
    Trim,

    /// # About
    /// **Specification:** SQL 2016
    TrimArray,

    /// # About
    /// **Specification:** SQL 2016
    True,

    /// # About
    /// **Specification:** SQL 2016
    Truncate,


    /// # About
    /// **Specification:** SQL 2016
    Uescape,

    /// # About
    /// **Specification:** SQL 1986
    Union,

    /// # About
    /// **Specification:** SQL 1986
    Unique,

    /// # About
    /// **Specification:** SQL 2016
    Unknown,

    /// # About
    /// **Specification:** SQL 2016
    Unnest,

    /// # About
    /// **Specification:** SQL 1986
    Update,

    /// # About
    /// **Specification:** SQL 1999
    Upper,

    /// # About
    /// **Specification:** SQL 1986
    User,

    /// # About
    /// **Specification:** SQL 2016
    Using,


    /// # About
    /// **Specification:** SQL 2016
    Value,

    /// # About
    /// **Specification:** SQL 1986
    Values,

    /// # About
    /// **Specification:** SQL 2016
    ValueOf,

    /// # About
    /// **Specification:** SQL 2016
    VarPop,

    /// # About
    /// **Specification:** SQL 2016
    VarSamp,

    /// # About
    /// **Specification:** SQL 2016
    Varbinary,

    /// # About
    /// **Specification:** SQL 2016
    Varchar,

    /// # About
    /// **Specification:** SQL 2016
    Varying,

    /// # About
    /// **Specification:** SQL 2016
    Versioning,


    /// # About
    /// **Specification:** SQL 2016
    When,

    /// # About
    /// **Specification:** SQL 1986
    Whenever,

    /// # About
    /// **Specification:** SQL 1986
    Where,

    /// # About
    /// **Specification:** SQL 2016
    WidthBucket,

    /// # About
    /// **Specification:** SQL 2016
    Window,

    /// # About
    /// **Specification:** SQL 1986
    With,

    /// # About
    /// **Specification:** SQL 2016
    Within,

    /// # About
    /// **Specification:** SQL 2016
    Without,


    /// # About
    /// **Specification:** SQL 2016
    Year,
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use strum::IntoEnumIterator;

    use super::*;

    /// Checks for overlap between non-reserved words and reserved words. This
    /// is disallowed since the lexer will parse them incorrectly, and the
    /// parser depends on the output of the lexer.
    #[test]
    fn no_overlap_between_non_reserved_words_and_reserved_words() {
        let reserved: HashSet<String> = ReservedWord::iter()
            .map(|word| word.to_string())
            .collect();

        let non_reserved: HashSet<String> = NonReservedWord::iter()
            .map(|word| word.to_string())
            .collect();

        let mut overlap: Vec<&str> = non_reserved.intersection(&reserved).map(|str| str.as_ref()).collect();
        overlap.sort();

        if !overlap.is_empty() {
            panic!("There is overlap in reserved and non-reserved words for {} word(s): {}",
                overlap.len(), overlap.join(", "));
        }
    }

    const SQL_2016_EBNF_NON_RESERVED_WORDS: &str = "    A | ABSOLUTE | ACTION | ADA | ADD | ADMIN | AFTER | ALWAYS | ASC
    | ASSERTION | ASSIGNMENT | ATTRIBUTE | ATTRIBUTES

    | BEFORE | BERNOULLI | BREADTH

    | C | CASCADE | CATALOG | CATALOG_NAME | CHAIN | CHAINING | CHARACTER_SET_CATALOG
    | CHARACTER_SET_NAME | CHARACTER_SET_SCHEMA | CHARACTERISTICS | CHARACTERS
    | CLASS_ORIGIN | COBOL | COLLATION | COLLATION_CATALOG | COLLATION_NAME | COLLATION_SCHEMA
    | COLUMNS | COLUMN_NAME | COMMAND_FUNCTION | COMMAND_FUNCTION_CODE | COMMITTED
    | CONDITIONAL | CONDITION_NUMBER | CONNECTION | CONNECTION_NAME | CONSTRAINT_CATALOG
    | CONSTRAINT_NAME | CONSTRAINT_SCHEMA | CONSTRAINTS | CONSTRUCTOR
    | CONTINUE | CURSOR_NAME

    | DATA | DATETIME_INTERVAL_CODE | DATETIME_INTERVAL_PRECISION | DEFAULTS | DEFERRABLE
    | DEFERRED | DEFINED | DEFINER | DEGREE | DEPTH | DERIVED | DESC | DESCRIBE_CATALOG
    | DESCRIBE_NAME | DESCRIBE_PROCEDURE_SPECIFIC_CATALOG
    | DESCRIBE_PROCEDURE_SPECIFIC_NAME | DESCRIBE_PROCEDURE_SPECIFIC_SCHEMA
    | DESCRIBE_SCHEMA | DESCRIPTOR
    | DIAGNOSTICS | DISPATCH | DOMAIN | DYNAMIC_FUNCTION | DYNAMIC_FUNCTION_CODE

    | ENCODING | ENFORCED | ERROR | EXCLUDE | EXCLUDING | EXPRESSION

    | FINAL | FINISH | FINISH_CATALOG | FINISH_NAME | FINISH_PROCEDURE_SPECIFIC_CATALOG
    | FINISH_PROCEDURE_SPECIFIC_NAME | FINISH_PROCEDURE_SPECIFIC_SCHEMA
    | FINISH_SCHEMA | FIRST | FLAG | FOLLOWING | FORMAT | FORTRAN | FOUND | FULFILL
    | FULFILL_CATALOG | FULFILL_NAME | FULFILL_PROCEDURE_SPECIFIC_CATALOG
    | FULFILL_PROCEDURE_SPECIFIC_NAME | FULFILL_PROCEDURE_SPECIFIC_SCHEMA | FULFILL_SCHEMA

    | G | GENERAL | GENERATED | GO | GOTO | GRANTED

    | HAS_PASS_THROUGH_COLUMNS | HAS_PASS_THRU_COLS | HIERARCHY

    | IGNORE | IMMEDIATE | IMMEDIATELY | IMPLEMENTATION | INCLUDING | INCREMENT | INITIALLY
    | INPUT | INSTANCE | INSTANTIABLE | INSTEAD | INVOKER | ISOLATION | IS_PRUNABLE

    | JSON

    | K | KEEP | KEY | KEYS | KEY_MEMBER | KEY_TYPE

    | LAST | LENGTH | LEVEL | LOCATOR

    | M | MAP | MATCHED | MAXVALUE | MESSAGE_LENGTH | MESSAGE_OCTET_LENGTH
    | MESSAGE_TEXT | MINVALUE | MORE | MUMPS

    | NAME | NAMES | NESTED | NESTING | NEXT | NFC | NFD | NFKC | NFKD
    | NORMALIZED | NULLABLE | NULLS | NUMBER

    | OBJECT | OCTETS | OPTION | OPTIONS | ORDERING | ORDINALITY | OTHERS
    | OUTPUT | OVERFLOW | OVERRIDING

    | P | PAD | PARAMETER_MODE | PARAMETER_NAME | PARAMETER_ORDINAL_POSITION
    | PARAMETER_SPECIFIC_CATALOG | PARAMETER_SPECIFIC_NAME | PARAMETER_SPECIFIC_SCHEMA
    | PARTIAL | PASCAL | PASS | PASSING | PAST | PATH | PLACING | PLAN | PLI
    | PRECEDING | PRESERVE | PRIOR | PRIVATE | PRIVATE_PARAMETERS | PRIVATE_PARAMS_S
    | PRIVILEGES | PRUNE | PUBLIC

    | QUOTES

    | READ | RELATIVE | REPEATABLE | RESPECT | RESTART | RESTRICT | RETURNED_CARDINALITY
    | RETURNED_LENGTH | RETURNED_OCTET_LENGTH | RETURNED_SQLSTATE | RETURNING
    | RETURNS_ONLY_PASS_THROUGH | RET_ONLY_PASS_THRU | ROLE | ROUTINE
    | ROUTINE_CATALOG | ROUTINE_NAME | ROUTINE_SCHEMA | ROW_COUNT

    | SCALAR | SCALE | SCHEMA | SCHEMA_NAME | SCOPE_CATALOG | SCOPE_NAME
    | SCOPE_SCHEMA | SECTION | SECURITY | SELF | SEQUENCE | SERIALIZABLE | SERVER_NAME
    | SESSION | SETS | SIMPLE | SIZE | SOURCE | SPACE | SPECIFIC_NAME | START_CATALOG
    | START_NAME | START_PROCEDURE_SPECIFIC_CATALOG | START_PROCEDURE_SPECIFIC_NAME
    | START_PROCEDURE_SPECIFIC_SCHEMA | START_SCHEMA | STATE | STATEMENT
    | STRING | STRUCTURE | STYLE | SUBCLASS_ORIGIN

    | T | TABLE_NAME | TABLE_SEMANTICS | TEMPORARY | THROUGH | TIES | TOP_LEVEL_COUNT
    | TRANSACTION | TRANSACTION_ACTIVE | TRANSACTIONS_COMMITTED
    | TRANSACTIONS_ROLLED_BACK | TRANSFORM | TRANSFORMS | TRIGGER_CATALOG | TRIGGER_NAME
    | TRIGGER_SCHEMA | TYPE

    | UNBOUNDED | UNCOMMITTED | UNCONDITIONAL | UNDER | UNNAMED | USAGE
    | USER_DEFINED_TYPE_CATALOG | USER_DEFINED_TYPE_CODE | USER_DEFINED_TYPE_NAME
    | USER_DEFINED_TYPE_SCHEMA | UTF16 | UTF32 | UTF8

    | VIEW

    | WORK | WRAPPER | WRITE

    | ZONE";

    /// Ensure all the non-reserved words of SQL 2016 are in registered in the
    /// [`NonReservedWords`] enum.
    #[test]
    fn non_reserved_words_sql_2016_ebnf() {
        let words: HashSet<&str> = SQL_2016_EBNF_NON_RESERVED_WORDS
            .split("|")
            .map(|word| word.trim())
            .collect();

        let enum_words: Vec<String> = NonReservedWord::iter()
            .map(|word| word.to_string())
            .collect();

        let enum_words: HashSet<&str> = enum_words.iter().map(|word| word.as_ref()).collect();

        for word in words.difference(&enum_words) {
            panic!("<non-reserved word> {word} from SQL 2016 not found in [`NonReservedWord`] enum!");
        }

        let mut difference = enum_words.difference(&words).map(|word| *word).collect::<Vec<&str>>();
        difference.sort();

        if !difference.is_empty() {
            panic!("<non-reserved word> {} found in [`NonReservedWord`] enum, but are not SQL 2016!",
                difference.join(", "));
        }
    }

    const SQL_2016_EBNF_RESERVED_WORDS: &str = "    ABS | ACOS | ALL | ALLOCATE | ALTER | AND | ANY | ARE | ARRAY | ARRAY_AGG
    | ARRAY_MAX_CARDINALITY | AS | ASENSITIVE | ASIN | ASYMMETRIC | AT | ATAN
    | ATOMIC | AUTHORIZATION | AVG

    | BEGIN | BEGIN_FRAME | BEGIN_PARTITION | BETWEEN | BIGINT | BINARY
    | BLOB | BOOLEAN | BOTH | BY

    | CALL | CALLED | CARDINALITY | CASCADED | CASE | CAST | CEIL | CEILING
    | CHAR | CHAR_LENGTH | CHARACTER | CHARACTER_LENGTH | CHECK | CLASSIFIER | CLOB
    | CLOSE | COALESCE | COLLATE | COLLECT | COLUMN | COMMIT | CONDITION | CONNECT
    | CONSTRAINT | CONTAINS | CONVERT | COPY | CORR | CORRESPONDING | COS | COSH
    | COUNT | COVAR_POP | COVAR_SAMP | CREATE | CROSS | CUBE | CUME_DIST | CURRENT
    | CURRENT_CATALOG | CURRENT_DATE | CURRENT_DEFAULT_TRANSFORM_GROUP
    | CURRENT_PATH | CURRENT_ROLE | CURRENT_ROW | CURRENT_SCHEMA | CURRENT_TIME
    | CURRENT_TIMESTAMP | CURRENT_PATH | CURRENT_ROLE | CURRENT_TRANSFORM_GROUP_FOR_TYPE
    | CURRENT_USER | CURSOR | CYCLE

    | DATE | DAY | DEALLOCATE | DEC | DECIMAL | DECFLOAT | DECLARE | DEFAULT | DEFINE
    | DELETE | DENSE_RANK | DEREF | DESCRIBE | DETERMINISTIC | DISCONNECT | DISTINCT
    | DOUBLE | DROP | DYNAMIC

    | EACH | ELEMENT | ELSE | EMPTY | END | END_FRAME | END_PARTITION | END-EXEC
    | EQUALS | ESCAPE | EVERY | EXCEPT | EXEC | EXECUTE | EXISTS | EXP
    | EXTERNAL | EXTRACT

    | FALSE | FETCH | FILTER | FIRST_VALUE | FLOAT | FLOOR | FOR | FOREIGN
    | FRAME_ROW | FREE | FROM | FULL | FUNCTION | FUSION

    | GET | GLOBAL | GRANT | GROUP | GROUPING | GROUPS

    | HAVING | HOLD | HOUR

    | IDENTITY | IN | INDICATOR | INITIAL | INNER | INOUT | INSENSITIVE | INSERT
    | INT | INTEGER | INTERSECT | INTERSECTION | INTERVAL | INTO | IS

    | JOIN | JSON_ARRAY | JSON_ARRAYAGG | JSON_EXISTS | JSON_OBJECT
    | JSON_OBJECTAGG | JSON_QUERY | JSON_TABLE | JSON_TABLE_PRIMITIVE | JSON_VALUE

    | LAG | LANGUAGE | LARGE | LAST_VALUE | LATERAL | LEAD | LEADING | LEFT | LIKE
    | LIKE_REGEX | LISTAGG | LN | LOCAL | LOCALTIME | LOCALTIMESTAMP | LOG | LOG10 | LOWER

    | MATCH | MATCH_NUMBER | MATCH_RECOGNIZE | MATCHES | MAX | MEMBER
    | MERGE | METHOD | MIN | MINUTE | MOD | MODIFIES | MODULE | MONTH | MULTISET

    | NATIONAL | NATURAL | NCHAR | NCLOB | NEW | NO | NONE | NORMALIZE | NOT
    | NTH_VALUE | NTILE | NULL | NULLIF | NUMERIC

    | OCTET_LENGTH | OCCURRENCES_REGEX | OF | OFFSET | OLD | OMIT | ON | ONE
    | ONLY | OPEN | OR | ORDER | OUT | OUTER | OVER | OVERLAPS | OVERLAY

    | PARAMETER | PARTITION | PATTERN | PER | PERCENT | PERCENT_RANK
    | PERCENTILE_CONT | PERCENTILE_DISC | PERIOD | PORTION | POSITION | POSITION_REGEX
    | POWER | PRECEDES | PRECISION | PREPARE | PRIMARY | PROCEDURE | PTF

    | RANGE | RANK | READS | REAL | RECURSIVE | REF | REFERENCES | REFERENCING
    | REGR_AVGX | REGR_AVGY | REGR_COUNT | REGR_INTERCEPT | REGR_R2 | REGR_SLOPE
    | REGR_SXX | REGR_SXY | REGR_SYY | RELEASE | RESULT | RETURN | RETURNS
    | REVOKE | RIGHT | ROLLBACK | ROLLUP | ROW | ROW_NUMBER | ROWS | RUNNING

    | SAVEPOINT | SCOPE | SCROLL | SEARCH | SECOND | SEEK | SELECT | SENSITIVE
    | SESSION_USER | SET | SHOW | SIMILAR | SIN | SINH | SKIP | SMALLINT | SOME | SPECIFIC
    | SPECIFICTYPE | SQL | SQLEXCEPTION | SQLSTATE | SQLWARNING | SQRT | START
    | STATIC | STDDEV_POP | STDDEV_SAMP | SUBMULTISET | SUBSET | SUBSTRING
    | SUBSTRING_REGEX | SUCCEEDS | SUM | SYMMETRIC | SYSTEM | SYSTEM_TIME
    | SYSTEM_USER

    | TABLE | TABLESAMPLE | TAN | TANH | THEN | TIME | TIMESTAMP | TIMEZONE_HOUR
    | TIMEZONE_MINUTE | TO | TRAILING | TRANSLATE | TRANSLATE_REGEX | TRANSLATION | TREAT
    | TRIGGER | TRIM | TRIM_ARRAY | TRUE | TRUNCATE

    | UESCAPE | UNION | UNIQUE | UNKNOWN | UNNEST | UPDATE    | UPPER | USER | USING

    | VALUE | VALUES | VALUE_OF | VAR_POP | VAR_SAMP | VARBINARY
    | VARCHAR | VARYING | VERSIONING

    | WHEN | WHENEVER | WHERE | WIDTH_BUCKET | WINDOW | WITH | WITHIN | WITHOUT

    | YEAR";

    /// Ensure all the reserved words of SQL 2016 are in registered in the
    /// [`super::ReservedWords`] enum.
    #[test]
    fn reserved_words_sql_2016_ebnf() {
        let words: HashSet<&str> = SQL_2016_EBNF_RESERVED_WORDS
            .split("|")
            .map(|word| word.trim())
            .collect();

        let enum_words: Vec<String> = ReservedWord::iter()
            .map(|word| word.to_string())
            .collect();

        let enum_words: HashSet<&str> = enum_words.iter().map(|word| word.as_ref()).collect();

        let mut difference = words.difference(&enum_words).map(|word| *word).collect::<Vec<&str>>();
        difference.sort();

        if !difference.is_empty() {
            panic!("<reserved word> {} from SQL 2016 not found in [`ReservedWord`] enum!",
                difference.join(", "));
        }

        // let mut difference = enum_words.difference(&words).map(|word| *word).collect::<Vec<&str>>();
        // difference.sort();

        // if !difference.is_empty() {
        //     panic!("<reserved word> {} found in [`ReservedWord`] enum, but are not SQL 2016!",
        //         difference.join(", "));
        // }
    }
}
