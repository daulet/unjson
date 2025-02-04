## Usage

```
kubectl logs $POD --tail 100 | unjson
```

Install: `brew tap daulet/unjson && brew install unjson`

## k9s

Copy [plugin.yaml](./k9s/plugin.yaml) to `~/Library/Application Support/k9s/plugins.yaml` (Mac) and add [kubectl-unjson](./k9s/kubectl-unjson) to your `$PATH`.
