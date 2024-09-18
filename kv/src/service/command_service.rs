use crate::*;

impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hgetall {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(v) => match store.set(&self.table, v.key, v.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into()
            }
            None => Value::default().into(),
        }
    }
}

impl CommandService for Hmset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        let pairs = self.pairs;
        let table = self.table;
        let response = pairs
            .into_iter()
            .map(|pair| {
                let result = store.set(&table, pair.key, pair.value.unwrap_or_default());
                match result {
                    Ok(Some(v)) => v,
                    _ => Value::default(),
                }
            })
            .collect::<Vec<_>>()
            .into();
        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_request::RequestData;

    #[test]
    fn hmset_should_work() {
        let store = MemTable::new();
        set_key_pairs("t1", vec![("u1", "world")], &store);
        let pairs = vec![
            Kvpair::new("u1", 10.1.into()),
            Kvpair::new("u2", 8.1.into()),
        ];
        let cmd = CommandRequest::new_hmset("t1", pairs);
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &["world".into(), Value::default()], &[]);
    }
    #[test]
    fn hset_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[Value::default()], &[]);

        let res = dispatch(cmd, &store);
        assert_res_ok(res, &["world".into()], &[]);
    }
    #[test]
    fn hget_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("t1", "hello", "world".into());
        dispatch(cmd, &store);
        let cmd = CommandRequest::new_hget("t1", "hello");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &["world".into()], &[]);
    }

    #[test]
    fn hgetall_should_work() {
        let store = MemTable::new();
        let cmds = vec![
            CommandRequest::new_hset("score", "u1", 10.into()),
            CommandRequest::new_hset("score", "u2", 8.into()),
            CommandRequest::new_hset("score", "u3", 11.into()),
            CommandRequest::new_hset("score", "u1", 6.into()),
        ];
        for cmd in cmds {
            dispatch(cmd, &store);
        }

        let cmd = CommandRequest::new_hgetall("score");
        let res = dispatch(cmd, &store);
        let pairs = &[
            Kvpair::new("u1", 6.into()),
            Kvpair::new("u2", 8.into()),
            Kvpair::new("u3", 11.into()),
        ];
        assert_res_ok(res, &[], pairs);
    }

    // 从Request中得到Response， 目前处理HGET/HGETALL/HSET
    fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
        match cmd.request_data.unwrap() {
            RequestData::Hget(v) => v.execute(store),
            RequestData::Hgetall(v) => v.execute(store),
            RequestData::Hset(v) => v.execute(store),
            RequestData::Hmset(v) => v.execute(store),
            _ => todo!()
        }
    }

    // 测试成功返回的结果
    fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
        res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(res.status, 200);
        assert_eq!(res.message, "");
        assert_eq!(res.values, values);
        assert_eq!(res.pairs, pairs);
    }
    // 测试失败返回的结果
    fn assert_res_error(res: CommandResponse, code: u32, msg: &str) {
        assert_eq!(res.status, code);
        assert!(res.message.contains(msg));
        assert_eq!(res.values, &[]);
        assert_eq!(res.pairs, &[]);
    }
}

fn set_key_pairs<T: Into<Value>>(table: &str, pairs: Vec<(&str, T)>, store: &impl Storage) {
    pairs
        .into_iter()
        .map(|(k, v)| CommandRequest::new_hset(table, k, v.into()))
        .for_each(|cmd| {
            dispatch(cmd, store);
        });
}