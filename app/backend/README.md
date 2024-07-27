# Setup

```
make setup
# Fill in values in the generated runtime/app_env
```

# Running 

```
make rundocker
```

If you want to have more control over the parts of the app, you can run them separately.
You can also run the app locally and have e.g. only database started in a container.
Look at `app/backend/Makefile` to see what options are available.
