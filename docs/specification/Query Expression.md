# `<query expression>`
* `<cursor specification>`:
    ```text
        <cursor specification> ::=
            <query expression> [ <order by clause> ]
            [ <updatability clause> ]
    ```
* `<from subquery>`
    ```text
        <from subquery> ::=
            [ <left paren> <insert column list> <right paren> ]
            [ override clause> ]
            <query expression>
    ```
* `<lateral derived table>`
    ```text
    LATERAL <left paren> <query expression> <right paren>
    ```


# `<derived table>`
is `<table subquery>`
    is `<subquery>`
        is `<left paren> <query expression> <right paren>`
