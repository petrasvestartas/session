# session_py

### Create and remove conda environment

```bash
conda env remove --name session
conda create --name session -c conda-forge compas python=3.13
```

### Test

```bash
pytest --doctest-modules src/session_py/ -v 
```
