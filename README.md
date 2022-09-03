# rompt

## what?

single standalone binary for a shell prompt, blisteringly fast, no config file, no daemon. needs a powerline font.

## why?

someday I'll learn to cleanly manage powerline installs, but today is not that day. until then, we do it ourselves.

## how?

```bash
# bash
export PS1="$(rompt \\u \\w)"
```
