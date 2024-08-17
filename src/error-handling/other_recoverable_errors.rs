#[allow(unused)]
fn main() {
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
    enum Ingredient {
        Avocado,
        Bacon,
        Egg,
    }

    #[derive(Debug, Eq, PartialEq)]
    struct BaconAndEggs;

    #[derive(Debug, Eq, PartialEq)]
    enum PreparationError {
        NotEnoughIngredient {
            ingredient: Ingredient,
            required: u32,
            available: u32,
        },
    }

    fn ensure_available(
        stock: &HashMap<Ingredient, u32>,
        ingredient: Ingredient,
        required: u32,
    ) -> Result<(), PreparationError> {
        let available = stock.get(&ingredient).copied().unwrap_or_default();

        if available >= required {
            Ok(())
        } else {
            Err(PreparationError::NotEnoughIngredient {
                ingredient,
                required,
                available,
            })
        }
    }

    fn prepare_bacon_and_eggs(
        stock: &mut HashMap<Ingredient, u32>,
    ) -> Result<BaconAndEggs, PreparationError> {
        const REQUIRED: [(Ingredient, u32); 2] = [(Ingredient::Bacon, 1), (Ingredient::Egg, 2)];

        for (ingredient, required) in REQUIRED {
            ensure_available(stock, ingredient, required)?;
        }

        for (ingredient, required) in REQUIRED {
            *stock.get_mut(&ingredient).unwrap() -= required;
        }

        Ok(BaconAndEggs)
    }

    let mut stock: HashMap<Ingredient, u32> = [
        (Ingredient::Avocado, 3),
        (Ingredient::Bacon, 2),
        (Ingredient::Egg, 1),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();
    // .collect();

    // println!("{stock:?}");
    assert_eq!(
        prepare_bacon_and_eggs(&mut stock),
        Err(PreparationError::NotEnoughIngredient {
            ingredient: Ingredient::Egg,
            required: 2,
            available: 1
        })
    );

    *stock.get_mut(&Ingredient::Egg).unwrap() += 6;

    assert_eq!(prepare_bacon_and_eggs(&mut stock), Ok(BaconAndEggs));

    assert_eq!(
        stock,
        [
            (Ingredient::Avocado, 3),
            (Ingredient::Bacon, 1),
            (Ingredient::Egg, 4),
        ]
        .into_iter()
        // .into()
        .collect::<HashMap<_, _>>()
    );
}
