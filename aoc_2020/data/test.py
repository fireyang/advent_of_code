file1 = open("day1_part1.txt", "r")
Lines = file1.readlines()

count = 0
# Strips the newline character
nums = []
for line in Lines:
    k = int(line)
    nums.append(k)

print(len(nums))

size = len(nums)
i = 0; j=0
import itertools

count = 0
for i, j in zip(nums, nums):
    print(i, j)
    count += 1

print(count)

#  while i < size:
#      j = i+1
#      while j < size:
#          k = j + 1
#          while k < size:
#              if(nums[i]+ nums[j] + nums[k]== 2020):
#                  print(i, j, k, nums[i] * nums[j]* nums[k])
#              k +=1
#          j+=1
#      i+=1


