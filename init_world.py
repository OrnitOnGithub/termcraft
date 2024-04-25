import random
worldsize = 3
# Create a test world of 100% grass
world_size = worldsize*worldsize*worldsize
print("creating "+ str(world_size) + " sized world")
with open("world.rmc", "w") as file:
  file.write("")
with open("world.rmc", "a") as file:
  for _ in range(world_size):
    file.write(str(random.randint(1,2)))
