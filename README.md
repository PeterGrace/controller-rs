## operator-rs
A kubernetes operator using reflectors in rust.

## Requirements
A kube cluster with access to read crds:

```sh
kubectl apply -f yaml/access.yaml
```

Custom resources installed in cluster:

```sh
kubectl apply -f yaml/examplecrd.yaml
kubectl apply -f yaml/crd-baz.yaml
```

Then you can run this by impersonating the `foobar` service account in `kube-system`.


## Usage

```sh
cargo run
```

Then you can try to remove a `foo`:

```sh
kubectl delete foo baz
```

and watch that the reflector picks up on in:

```
[2019-04-28T22:03:08Z INFO  operator::kube] Removing service baz
```

ditto if you try to apply one:

```sh
kubectl apply -f yaml/crd-qux.yaml
```

```
[2019-04-28T22:07:01Z INFO  operator::kube] Adding service baz
```

If you edit, and then apply, baz, you'll get:

```
[2019-04-28T22:08:21Z INFO  operator::kube] Modifying service baz
```

In all cases, the reflector maintains an internal state for the `foos` custom resource.

