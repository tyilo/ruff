def unclosed[A, *B(test: name):
    pass

def keyword[A, await](): ...

def not_a_type_param[A, 100, B](): ...

def multiple_commas[A,,B](): ...

def multiple_trailing_commas[A,,](): ...

def multiple_commas_and_recovery[A,,100](): ...