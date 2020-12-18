Having an IPFS-cli installed on your machine is a requirement: https://docs.ipfs.io/install/command-line/#package-managers

Start development server with live code reloaing:
```
npm run dev
```

Build and export static pages to be uploaded to IPFS:
```
npm run export
```

Start local node + gateway in background:
```
ipfs daemon
```

To add files to IPFS:
```
ipfs add -r out/
```
Which will print out unique hashes for all the files inside `out/` dir. Copy the hash of the root level dir. (If you are not sure run the command: `ipfs files stat /out | head -1` to obtain the hash)

App will be available on:
```
localhost:8080/ipfs/<hash>
```
