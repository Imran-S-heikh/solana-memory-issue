## Memory allocation failed, out of memory

Here total account size is `30KB` (10KB with init, 20KB added later)  and Maximum heap size could be `32KB` (according to the doc). But in the test its only able to add only 496(`33 Each`) that means we are able to use around `16KB`. So why we are not able to use up to `30KB`. Why we get memory error just after adding `496 Items`. In According to the space reference we sould be able to store 929 Items. 

Is there anything that is using the `heap` memory ? Or I have issue in my calculation ?
