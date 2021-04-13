# pass-tutor
This program can generate and help you practice new passwords.

## Building
To build, simply use cargo:
```
cargo build
```

## Usage
You can start the program just by running it and it'll generate new passwords until you select the one you want. The default length is 12.
```
pass-tutor
```
Alternativelly, you can specify password generation criteria:
```
# Generate alphanumeric passwords of length 10 only
pass-tutor -nUL -l 10
```
Below is a table describing password generation options:
|Arg|Description|
|---|-----------|
|-s, --symbols|Allow symbols|
|-n, --numbers|Allow numbers|
|-U, --upper|Allow upper-case letters|
|-L, --lower|Allow lower-case letters|
|-l, --length|Specify password length|

## Contributing
Feel free to submit a pull request or post an issue with suggestions or improvements.
