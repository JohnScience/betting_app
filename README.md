# Crypto betting app

SDD: <https://docs.google.com/document/d/1U1PDZC_sZY5XeCc71WyDy0mWSrCJs-WU7Pba7gEZzaA/edit>

## Project structure

- `front` - the frontend part of the project. It is a [vite] JS+Rust(WASM) app.
- `app` - an [Apache Cordova] app that wraps the frontend part into a mobile app.

## Intended workflow

Change directory to the `front`:

```console
cd front
```

Assuming you have the dependencies installed, you can tweak the frontend part and see the changes.

```console
npm run dev
```

Once you are happy with the changes, you can build the frontend part for testing it on a mobile device:

```console
npm run build-for-app && cd ../app && cordova run <android|ios>
```

*TODO: add iOS target, possibly via [cordova-ios], and check if it works as intended.*

[vite]: https://vitejs.dev/
[Apache Cordova]: https://cordova.apache.org/
[cordova-ios]: https://github.com/apache/cordova-ios
