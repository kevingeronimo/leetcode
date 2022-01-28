class Solution:      
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        i = 0
        j = 0
        total = None
        
        while total != target:
            j += 1
            
            if j >= len(nums):
                i += 1
                j = i
            else:
                total = nums[i] + nums[j]
        
        return [i, j]