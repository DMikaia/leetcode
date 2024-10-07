/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

function isSameTree(p: TreeNode | null, q: TreeNode | null): boolean {
    let stack_p = [p];
    let stack_q = [q];

    while (stack_p.length > 0 && stack_p.length > 0) {
        let node_p = stack_p.pop();
        let node_q = stack_q.pop();

        if (node_p == null && node_q == null) {
            continue;
        }

        if (node_p == null || node_q == null || node_p.val != node_q.val) {
            return false;
        }

        stack_p.push(node_p.left);
        stack_p.push(node_p.right);
        stack_q.push(node_q.left);
        stack_q.push(node_q.right);
    }

    return stack_p.length === 0 && stack_q.length === 0;
};