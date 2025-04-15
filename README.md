
Provides a trivial tool which holds state - a counter.

This needs a local ollama serves to be running and the model "llama3.2:1b" to be loaded (`ollama pull llama3.2:1b`).

Example output:

```
what is the current state of the counter?
>Counter status: 0
The current state of the counter is 0. I'm here to help you with any tool-related questions or issues you might have. How can I assist you further?
can you increment the counter by one
>Counter status: 1
The current state of the counter is now 1. If you need to increment it by a different value, just let me know and I'll adjust the output accordingly!
```

However you will sometimes see errors where the model fails to produce input which fits the schema.
