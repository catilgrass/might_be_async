1.

2. ```rust
       select![
           "async" => { func_async().await },
           "sync" => { func_sync() }
       ];

       select![
           { func_async().await },
           { func_sync() }
       ];

       select![
           "async" => func_async().await,
           "sync" => func_sync()
       ];

       select![
           func_async().await,
           func_sync()
       ];

       select![
           "async" => { func_async().await },
           ! => { func_sync() }
       ];

       select![
           { func_async().await },
           { func_sync() }
       ];

       select![
           ! => func_async().await,
           "sync" => func_sync()
       ];

       select![
           func_sync(),
           func_async().await,
       ];
   ```

    说明，显式模式精确指定即可，隐式模式需要判断（是否有且只有一边有.await）
