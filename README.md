# Small Admin

NEAR Contract to test admin interface. Admin can be set to any NEAR account, including a [DAO](https://astrodao.com/).

## Compile

```
./build.sh
```

[Contract builder](https://github.com/near/near-sdk-rs/tree/master/contract-builder) is recommended for reproducing the same binary while compiling on different machines.

Final binary will be on `res/small_admin.wasm`

## Deploy

Following commands are using [near-cli](https://github.com/near/near-cli)

```
near deploy small_admin.near res/small_admin.wasm '{"owner": "the_real_admin.near"}'
```

Notice on deployment a Full Access Key is created for this contract. The contract itself is considered an admin, so in order to remove it, all Full Access Keys must be removed (it should be only one during deployment).

## View methods

### get_owner

```
near view small_admin.near get_owner
```

### get_counter

Everyone can see the counter but only admin can increase it.

```
near view small_admin.near get_counter
```

## Call methods

Only the owner can call the following methods.

### increase_counter

```
near call small_admin.near increase_counter --accountId the_real_admin.near
```

### set_owner

New owner can be any NEAR account id, including a DAO account. Notice this replace the previous owner for the new one.

```
near call small_admin.near set_owner '{"owner": "dao_admin.sputnik-dao.near"}' --accountId the_real_admin.near
```
