use crate::{
    resp::{BulkString, Value},
    tests::get_default_addr,
    ConnectionMultiplexer, DatabaseCommandResult, GenericCommands, LInsertWhere,
    LMoveWhere::Left,
    LMoveWhere::Right,
    ListCommands, Result,
};
use serial_test::serial;

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn lindex() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del("mylist").send().await?;

    database
        .rpush("mylist", ["element1", "element2", "element3"])
        .send().await?;

    let element: String = database.lindex("mylist", 0).send().await?;
    assert_eq!("element1", element);

    let element: String = database.lindex("mylist", -1).send().await?;
    assert_eq!("element3", element);

    let element: Value = database.lindex("mylist", 3).send().await?;
    assert!(matches!(element, Value::BulkString(BulkString::Nil)));

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn linsert() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del("mylist").send().await?;

    database.rpush("mylist", ["element1", "element3"]).send().await?;

    let result = database
        .linsert("mylist", LInsertWhere::After, "element1", "element2")
        .send().await?;
    assert_eq!(3, result);

    let elements: Vec<String> = database.lrange("mylist", 0, -1).send().await?;
    assert_eq!(3, elements.len());
    assert_eq!("element1".to_string(), elements[0]);
    assert_eq!("element2".to_string(), elements[1]);
    assert_eq!("element3".to_string(), elements[2]);

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn llen() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del("mylist").send().await?;

    database
        .rpush("mylist", ["element1", "element2", "element3"])
        .send().await?;

    let len = database.llen("mylist").send().await?;
    assert_eq!(3, len);

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn lmove() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del(["mylist", "myotherlist"]).send().await?;

    database
        .rpush("mylist", ["element1", "element2", "element3"])
        .send().await?;

    let element: String = database.lmove("mylist", "myotherlist", Right, Left).send().await?;
    assert_eq!("element3", element);

    let element: String = database.lmove("mylist", "myotherlist", Left, Right).send().await?;
    assert_eq!("element1", element);

    let elements: Vec<String> = database.lrange("mylist", 0, -1).send().await?;
    assert_eq!(1, elements.len());
    assert_eq!("element2".to_string(), elements[0]);

    let elements: Vec<String> = database.lrange("myotherlist", 0, -1).send().await?;
    assert_eq!(2, elements.len());
    assert_eq!("element3".to_string(), elements[0]);
    assert_eq!("element1".to_string(), elements[1]);

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn lmpop() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del("mylist").send().await?;

    database
        .lpush(
            "mylist",
            ["element1", "element2", "element3", "element4", "element5"],
        )
        .send().await?;

    let result: (String, Vec<String>) = database.lmpop("mylist", Left, 1).send().await?;
    assert_eq!("mylist", result.0);
    assert_eq!(1, result.1.len());
    assert_eq!("element5".to_string(), result.1[0]);

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn lpop() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del("mylist").send().await?;

    database
        .lpush("mylist", ["element1", "element2", "element3"])
        .send().await?;

    let elements: Vec<String> = database.lpop("mylist", 2).send().await?;
    assert_eq!(2, elements.len());
    assert_eq!("element3", elements[0].as_str());
    assert_eq!("element2", elements[1].as_str());

    let elements: Vec<String> = database.lpop("mylist", 1).send().await?;
    assert_eq!(1, elements.len());
    assert_eq!("element1", elements[0].as_str());

    let elements: Vec<String> = database.lpop("mylist", 1).send().await?;
    assert_eq!(0, elements.len());

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn lpos() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del("mylist").send().await?;

    database
        .rpush("mylist", ["element1", "element2", "element3"])
        .send().await?;

    let pos = database
        .lpos("mylist", "element2", Some(1), Some(1))
        .send().await?;
    assert_eq!(None, pos);

    let pos = database
        .lpos("mylist", "element2", Some(1), Some(3))
        .send().await?;
    assert_eq!(Some(1), pos);

    let pos: Vec<usize> = database
        .lpos_with_count("mylist", "element2", 1, Some(1), Some(1))
        .send().await?;
    assert_eq!(0, pos.len());

    let pos: Vec<usize> = database
        .lpos_with_count("mylist", "element2", 1, Some(1), Some(3))
        .send().await?;
    assert_eq!(1, pos.len());
    assert_eq!(1, pos[0]);

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn lpush() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del("mylist").send().await?;

    let size = database.lpush("mylist", "element1").send().await?;
    assert_eq!(1, size);

    let size = database.lpush("mylist", ["element2", "element3"]).send().await?;
    assert_eq!(3, size);

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn lpushx() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del("mylist").send().await?;

    let len = database.lpushx("mylist", "element1").send().await?;
    assert_eq!(0, len);

    database.lpush("mylist", "element1").send().await?;
    let len = database.lpush("mylist", "element2").send().await?;
    assert_eq!(2, len);

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn lrange() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del("mylist").send().await?;

    database
        .rpush("mylist", ["element1", "element2", "element3"])
        .send().await?;

    let elements: Vec<String> = database.lrange("mylist", 0, -1).send().await?;
    assert_eq!(3, elements.len());
    assert_eq!("element1".to_string(), elements[0]);
    assert_eq!("element2".to_string(), elements[1]);
    assert_eq!("element3".to_string(), elements[2]);

    let elements: Vec<String> = database.lrange("mylist", -2, 1).send().await?;
    assert_eq!(1, elements.len());
    assert_eq!("element2".to_string(), elements[0]);

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn lrem() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del("mylist").send().await?;

    database
        .rpush("mylist", ["element1", "element1", "element3"])
        .send().await?;

    let len = database.lrem("mylist", 3, "element1").send().await?;
    assert_eq!(2, len);

    let len = database.lrem("mylist", -1, "element1").send().await?;
    assert_eq!(0, len);

    let len = database.lrem("mylist", 0, "element3").send().await?;
    assert_eq!(1, len);

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn lset() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del("mylist").send().await?;

    database
        .rpush("mylist", ["element1", "element1", "element3"])
        .send().await?;

    database.lset("mylist", 0, "element4").send().await?;
    database.lset("mylist", -2, "element5").send().await?;

    let elements: Vec<String> = database.lrange("mylist", 0, -1).send().await?;
    assert_eq!(3, elements.len());
    assert_eq!("element4".to_string(), elements[0]);
    assert_eq!("element5".to_string(), elements[1]);
    assert_eq!("element3".to_string(), elements[2]);

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn ltrim() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del("mylist").send().await?;

    database
        .rpush("mylist", ["element1", "element2", "element3"])
        .send().await?;

    database.ltrim("mylist", 1, -1).send().await?;

    let elements: Vec<String> = database.lrange("mylist", 0, -1).send().await?;
    assert_eq!(2, elements.len());
    assert_eq!("element2".to_string(), elements[0]);
    assert_eq!("element3".to_string(), elements[1]);

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn rpop() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del("mylist").send().await?;

    database
        .rpush("mylist", ["element1", "element2", "element3"])
        .send().await?;

    let elements: Vec<String> = database.rpop("mylist", 2).send().await?;
    assert_eq!(2, elements.len());
    assert_eq!("element3", elements[0].as_str());
    assert_eq!("element2", elements[1].as_str());

    let elements: Vec<String> = database.rpop("mylist", 1).send().await?;
    assert_eq!(1, elements.len());
    assert_eq!("element1", elements[0].as_str());

    let elements: Vec<String> = database.rpop("mylist", 1).send().await?;
    assert_eq!(0, elements.len());

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn rpush() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del("mylist").send().await?;

    let len = database.rpush("mylist", "element1").send().await?;
    assert_eq!(1, len);

    let len = database.rpush("mylist", ["element2", "element3"]).send().await?;
    assert_eq!(3, len);

    Ok(())
}

#[cfg_attr(feature = "tokio-runtime", tokio::test)]
#[cfg_attr(feature = "async-std-runtime", async_std::test)]
#[serial]
async fn rpushx() -> Result<()> {
    let connection = ConnectionMultiplexer::connect(get_default_addr()).await?;
    let database = connection.get_default_database();

    // cleanup
    database.del(["mylist", "myotherlist"]).send().await?;

    database.rpush("mylist", "element1").send().await?;

    let len = database.rpushx("mylist", "element2").send().await?;
    assert_eq!(2, len);

    let len = database.rpushx("myotherlist", "element2").send().await?;
    assert_eq!(0, len);

    let elements: Vec<String> = database.lrange("mylist", 0, -1).send().await?;
    assert_eq!(2, elements.len());
    assert_eq!("element1".to_string(), elements[0]);
    assert_eq!("element2".to_string(), elements[1]);

    let elements: Vec<String> = database.lrange("myotherlist", 0, -1).send().await?;
    assert_eq!(0, elements.len());

    Ok(())
}
