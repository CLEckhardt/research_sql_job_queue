


## Experiment

### What data do we need to create and collect?

The database entries need:
* unique ID
* an owner that has claimed the resource (0 represents no owner)
* a resource name for easier tracking

The instances need:
* unique ID
* claimed resource
* number of attempts

We need multiple resources and some multiple of instances.

### How do we generate a positive state?

This is a state that is not possible unless the experiment worked.

#### What does a positive state look like?

* Each resource has a unique owner reflected in the database
* The database entries' owner field matches the instance with the corresponding
  claimed resource
* No two instances say they have cliamed the same resource
* The number of instances with a claimed resource equals the number of
  resources

In summary, a solution would provide a non-deterministic one-to-one mapping of
resources into instances, and this mapping is correctly reflected in the
instances' states.

### How would we generate a negative state?

This is a state that is only possible if the soluton does not work.
The control should generate this state reliably.

#### What does a negative state look like?

* Two instances say they have claimed the same resource
* There is a resource with no owner
* There are two resources with the same owner

### What does an valid state look like?

If these conditions are not true, there is something wrong with the experiment -
both a positive and negative state should have these characteristics.

* Every instance has a number of attempts > 0
* Every instance has either claimed a resource or its number of attempts equals
  the maximum number of attempts


We need to ensure that our solution:
* Generates the positive state
* Does not generate a negative state when the control reliably does
