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
    /// **Specification:** SQL 1986
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


    /// # About
    /// **Specification:** SQL 2003
    Before,

    /// # About
    /// **Specification:** SQL 2003
    Bernoulli,

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
    /// **Specification:** SQL 1999
    Contains,

    /// # About
    /// **Specification:** SQL 1986
    Continue,

    /// # About
    /// **Specification:** SQL 1999
    Convert,

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
    Existing,

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
    /// **Specification:** SQL 1986
    All,

    /// # About
    /// **Specification:** SQL 1986
    And,

    /// # About
    /// **Specification:** SQL 1986
    Any,

    /// # About
    /// **Specification:** SQL 1986
    As,

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
    /// **Specification:** SQL 1986
    Between,

    /// # About
    /// **Specification:** SQL 1986
    By,


    /// # About
    /// **Specification:** SQL 1986
    Char,

    /// # About
    /// **Specification:** SQL 1986
    Character,

    /// # About
    /// **Specification:** SQL 1986
    Check,

    /// # About
    /// **Specification:** SQL 1986
    Close,

    /// # About
    /// **Specification:** SQL 1986
    Commit,

    /// # About
    /// **Specification:** SQL 1986
    Count,

    /// # About
    /// **Specification:** SQL 1986
    Create,

    /// # About
    /// **Specification:** SQL 1986
    Current,

    /// # About
    /// **Specification:** SQL 1986
    Cursor,


    /// # About
    /// **Specification:** SQL 1986
    Dec,

    /// # About
    /// **Specification:** SQL 1986
    Decimal,

    /// # About
    /// **Specification:** SQL 1986
    Declare,

    /// # About
    /// **Specification:** SQL 1986
    Delete,

    /// # About
    /// **Specification:** SQL 1986
    Distinct,

    /// # About
    /// **Specification:** SQL 1986
    Double,


    /// # About
    /// **Specification:** SQL 1986
    End,

    /// # About
    /// **Specification:** SQL 1986
    Escape,

    /// # About
    /// **Specification:** SQL 1986
    Exec,

    /// # About
    /// **Specification:** SQL 1986
    Exists,


    /// # About
    /// **Specification:** SQL 1986
    Fetch,

    /// # About
    /// **Specification:** SQL 1986
    Float,

    /// # About
    /// **Specification:** SQL 1986
    For,

    /// # About
    /// **Specification:** SQL 1986
    From,

    /// # About
    /// **Specification:** SQL 1986
    Grant,

    /// # About
    /// **Specification:** SQL 1986
    Group,


    /// # About
    /// **Specification:** SQL 1986
    Having,


    /// # About
    /// **Specification:** SQL 1986
    In,

    /// # About
    /// **Specification:** SQL 1986
    Indicator,

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
    /// **Specification:** SQL 1986
    Into,

    /// # About
    /// **Specification:** SQL 1986
    Is,


    /// # About
    /// **Specification:** SQL 1986
    Language,

    /// # About
    /// **Specification:** SQL 1986
    Like,


    /// # About
    /// **Specification:** SQL 1986
    Max,

    /// # About
    /// **Specification:** SQL 1986
    Min,

    /// # About
    /// **Specification:** SQL 1986
    Module,


    /// # About
    /// **Specification:** SQL 1986
    Not,

    /// # About
    /// **Specification:** SQL 1986
    Null,

    /// # About
    /// **Specification:** SQL 1986
    Numeric,


    /// # About
    /// **Specification:** SQL 1986
    Of,

    /// # About
    /// **Specification:** SQL 1986
    On,

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
    /// **Specification:** SQL 1986
    Precision,

    /// # About
    /// **Specification:** SQL 1986
    Procedure,


    /// # About
    /// **Specification:** SQL 1986
    Real,

    /// # About
    /// **Specification:** SQL 1986
    Rollback,


    /// # About
    /// **Specification:** SQL 1986
    Select,

    /// # About
    /// **Specification:** SQL 1986
    Set,

    /// # About
    /// **Specification:** SQL 1986
    Smallint,

    /// # About
    /// **Specification:** SQL 1986
    Some,

    /// # About
    /// **Specification:** SQL 1986
    Sql,

    /// # About
    /// **Specification:** SQL 1986
    Sqlcode,

    /// # About
    /// **Specification:** SQL 1986
    Sqlerror,

    /// # About
    /// **Specification:** SQL 1986
    Sum,


    /// # About
    /// **Specification:** SQL 1986
    Table,

    /// # About
    /// **Specification:** SQL 1986
    To,


    /// # About
    /// **Specification:** SQL 1986
    Union,

    /// # About
    /// **Specification:** SQL 1986
    Unique,

    /// # About
    /// **Specification:** SQL 1986
    Update,

    /// # About
    /// **Specification:** SQL 1986
    User,


    /// # About
    /// **Specification:** SQL 1986
    Values,


    /// # About
    /// **Specification:** SQL 1986
    Whenever,

    /// # About
    /// **Specification:** SQL 1986
    Where,

    /// # About
    /// **Specification:** SQL 1986
    With,
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
        let words: HashSet<_> = SQL_2016_EBNF_NON_RESERVED_WORDS
            .split("|")
            .map(|word| word.trim())
            .collect();

        let enum_words: Vec<_> = NonReservedWord::iter()
            .map(|word| word.to_string())
            .collect();

        for word in words.difference(&enum_words.iter().map(|word| word.as_ref()).collect()) {
            if words.contains(word) {
                panic!("<non-reserved word> {word} from SQL 2016 not found in [`NonReservedWord`] enum!");
            } else {
                panic!("<non-reserved word> {word} found in [`NonReservedWord`] enum, but is not SQL 2016!");
            }
        }
    }
}
