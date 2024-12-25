import polars as pl
from polars import col as C

INPUT_CSV = "output/output.csv"

df = pl.read_csv(INPUT_CSV)

df = df.with_columns(
    pl.when(C("shiny").eq("true"))
    .then(pl.lit(True))
    .otherwise(pl.lit(False))
    .alias("shiny")
)

n_pokemon = df.height
shinies = df.filter(C("shiny")).height
shiny_rate = shinies / df.height
inverse_shiny_rate = 1 / shiny_rate
print(f"Number of Pokemon: {n_pokemon}")
print(f"Number of shinies: {shinies}")
print(f"Shiny rate: 1/{inverse_shiny_rate:.2f}")

iv_cols = [col for col in df.columns if col.endswith("_iv")]
df = df.with_columns(total_iv=pl.lit(0))
for col in iv_cols:
    df = df.with_columns(C("total_iv").add(C(col)))

print(df.select(iv_cols + ["total_iv"]).sort("total_iv"))
