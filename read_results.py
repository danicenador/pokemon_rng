import polars as pl
from polars import col as C

INPUT_CSV = "output/output.csv"

df = pl.read_csv(INPUT_CSV)

shinies = df.filter(C("shiny")).height
print(f"Number of Pokemon: {df.height}")
print(f"Number of shinies: {shinies}")

iv_cols = [col for col in df.columns if col.endswith("_iv")]
df = df.with_columns(total_iv=pl.lit(0))
for col in iv_cols:
    df = df.with_columns(C("total_iv").add(C(col)))

print(df.select(iv_cols + ["total_iv"]).sort("total_iv"))
