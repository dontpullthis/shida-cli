# Shida [UNDER DEVELOPMENT]

A data migration tool. As the project is under development now, the documentation below is just a concept.

## Usage

```bash
$ shida \
  --src-param="type=mysql" \
  --src-param="user=johndoe" \
  --src-param="password=qwerty" \
  --src-param="host=hostname" \
  --src-param="port=3306" \
  --src-param="database=db_name" \
  --dest-param="type=postgresql" \
  --dest-param="user=johndoe" \
  --dest-param="password=qwerty" \
  --dest-param="host=hostname" \
  --dest-param="port=5432" \
  --dest-param="database=db_name"
```