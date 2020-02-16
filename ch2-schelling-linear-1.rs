struct Individual {
  preference:
  class: String,
}

enum House {
  Occupied(Individual),
  Vacant
}

type Community = Vec(House);
