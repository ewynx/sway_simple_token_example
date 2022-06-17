# Fuel Sway | Simple token example

The [Sway example](https://fuellabs.github.io/sway/latest/examples/subcurrency.html) from documentation with a few tests. 

## Run tests
```
forc build
forc test
```

## Notes

I'm stuck on:
- logging. In test it worked for a while to use "println!" which is now magically not working anymore. In contract apparently it should be with log(), but I don't see that appear in the console at all.
- choosing a sending address in test. I can't figure out IF this is possible and HOW. In this specific example we need it to test whether we can mint being the minter, as well as using "send" (now the balance of the sender is always 0, because I don't know who it is)

