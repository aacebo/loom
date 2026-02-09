# Loom CLI Backlog

See the [global backlog](/backlog/README.md) for all planned work.

## Improvements

### Std Out

CLI needs to output more information about the command results,
the input file/sample metadata, and the progress indicator should ideally
write file results incrementally over time so that if a failure happens we still have
some results.

## Bugs

### Output

outputs are being defaulted to the sample filename, so if I run loom score datasets/label.samples.json the output
defaults to datasets/label.samples.json/results.json, when it should be datasets/label.results.json.
