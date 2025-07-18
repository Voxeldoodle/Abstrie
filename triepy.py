class Node:
    def __init__(self, prefixes=set(['']), pref_length=None):
        self.pref_length = pref_length if pref_length else len(next(iter(prefixes)))
        # self.pref_length = min(len(s) for s in prefixes)
        # self.pref_length = len(next(iter(prefixes)))
        self.prefixes    = prefixes
    def add_prefix(self,pref):
        self.prefixes |= set([pref])

    def __eq__(self, other):
        if isinstance(other, Node):
            return self.pref_length == other.pref_length #and self.prefixes == other.prefixes
        return False

    def __hash__(self):
        return hash(self.pref_length)

    def __str__(self):
        return f"{self.pref_length} {self.prefixes}"
    
    def get_regex(self):
        return rgxge.to_fuzzy_regEx(list(self.prefixes))
    
    # import regexgen as rgxge
    # def __str__(self):
    #     return f"{self.pref_length} {rgxge.to_fuzzy_regEx(list(self.prefixes))} {self.prefixes}"


node1 = Node(pref_length=1)
node2 = Node(pref_length=2)

my_dict = {node1: node2}

print(my_dict[Node(pref_length=1)])

class PrefixTree:
    def __init__(self):
        self.root = {}

    def insert(self, word):
        node = self.root
        for char in word:
            if char not in node:
                node[char] = {}
            node = node[char]
        node['$'] = True
    
    def get_prefixes_tree(self):
        def update_key(my_dict, chk, value):
            for k in my_dict.keys():
                if k == chk:
                    k.add_prefix(value)
        def get_key(my_dict, chk):
            for k in my_dict.keys():
                if k == chk:
                    return k
            return None
        lengths = {}
        stack = [(self.root, "")]
        root_stack = ['']
        
        while stack:
            node, prefix = stack.pop()
            if len(node) > 1 and prefix != '':
                pre = root_stack.pop()
                while not prefix.startswith(pre):                    
                    pre = root_stack.pop()
                tmp_l = lengths
                for r in root_stack[1:]:
                    l_r = Node(pref_length=len(r))
                    # print(f'Loop: "{r}", {prefix}, "{root_stack}"')#, {[str(x) for x in tmp_l.keys()]}, {[str(x) for x in lengths.keys()]}')
                    # print_tree(tmp_l)
                    if l_r not in tmp_l:
                        print('Node: ',Node(set([r])))
                        tmp_l[Node(set([r]))] = {}
                    else:
                        if isinstance(tmp_l[l_r], int):
                            tmp_l[l_r] = {tmp_l[l_r]: -1}
                    tmp_l = tmp_l[l_r]
                    
                # print(f"Out: '{pre}', {prefix}, {root_stack}")#,{tmp_l}, {prefixes}")
                # if len(tmp_l.keys())>0:
                #     print_tree(tmp_l)
                
                if pre:
                    n_pre = Node(pref_length=len(pre))
                    if isinstance(tmp_l[n_pre], dict):
                            # tmp[pre] = {tmp[pre]:'$'}
                            tmp_l = tmp_l[n_pre]
                            if Node(set([prefix])) not in tmp_l:
                                tmp_l[Node(set([prefix]))] = -1
                    else:
                        tmp_l[n_pre] = {Node(set([prefix])):-1}
                    # print(f"If pre: '{pre}', {prefix}, {root_stack}")#,{tmp_l}, {prefixes}")
                    # if len(tmp_l.keys())>0:
                    #     print_tree(tmp_l)
                else:
                    if Node(set([prefix])) not in tmp_l:
                        tmp_l[Node(set([prefix]))] = -1
                update_key(tmp_l, Node(set([prefix])), prefix)
                    
                root_stack.append(pre)
                root_stack.append(prefix)                
                # print(root_stack)
            for char, child_node in node.items():
                if char != '$':
                    stack.append((child_node, prefix + char))
        return lengths
    
    def get_prefixes(self):
        prefixes = []
        stack = [(self.root, "")]
        root_stack = ['']
        
        while stack:
            node, prefix = stack.pop()
            if len(node) > 1 and prefix != '':
                pre = root_stack.pop()
                while not prefix.startswith(pre):                    
                    pre = root_stack.pop()
                prefixes.append((pre, prefix)) # Consider making a dictonary instead
                root_stack.append(pre)
                root_stack.append(prefix)                
                # print(root_stack)
            for char, child_node in node.items():
                if char != '$':
                    stack.append((child_node, prefix + char))
        return prefixes
    
    def get_prefixes_dict(self):
        prefixes = {}
        lengths = {}
        stack = [(self.root, "")]
        root_stack = ['']
        
        while stack:
            node, prefix = stack.pop()
            if len(node) > 1 and prefix != '':
                pre = root_stack.pop()
                while not prefix.startswith(pre):                    
                    pre = root_stack.pop()
                tmp_p = prefixes
                tmp_l = lengths
                for r in root_stack[1:]:
                    l_r = len(r)
                    # print(f'Loop: "{r}", {prefix}, "{root_stack}", {tmp}, {prefixes}')
                    if r not in tmp_p:
                        tmp_p[r] = {}
                    else:
                        if isinstance(tmp_p[r], str):
                            tmp_p[r] = {tmp_p[r]:'$'}
                    if l_r not in tmp_l:
                        tmp_l[l_r] = {}
                    else:
                        if isinstance(tmp_l[l_r], int):
                            tmp_l[l_r] = {tmp_l[l_r]: -1}
                    tmp_p = tmp_p[r]
                    tmp_l = tmp_l[l_r]
                # print(f"Out: '{pre}', {prefix},{tmp}, {prefixes}")
                if pre:
                    # print(f"In: '{pre}', {prefix},{tmp_p}, {prefixes}")
                    # In: 'ba', ban,{'ba': {'bat': '$'}}, {'ba': {'bat': '$'}}
                    if isinstance(tmp_p[pre], dict):
                            # tmp[pre] = {tmp[pre]:'$'}
                            tmp_p = tmp_p[pre]
                            if prefix not in tmp_p:
                                tmp_p[prefix] = '$'
                    else:
                        tmp_p[pre] = {prefix:'$'}
                    if isinstance(tmp_l[len(pre)], dict):
                            tmp_l = tmp_l[len(pre)]
                            if len(prefix) not in tmp_l:
                                tmp_l[len(prefix)] = -1
                    else:
                        tmp_l[len(pre)] = {len(prefix):-1}
                else:
                    if prefix not in tmp_p:
                        tmp_p[prefix] = '$'
                    if len(prefix) not in tmp_l:
                        tmp_l[len(prefix)] = -1
                    
                root_stack.append(pre)
                root_stack.append(prefix)                
                # print(root_stack)
            for char, child_node in node.items():
                if char != '$':
                    stack.append((child_node, prefix + char))
        return [prefixes, lengths]
    
def print_tree(nodes, indent=''):
    keys = list(nodes.keys())
    last_key = keys[-1]

    for key, value in nodes.items():
        if key == last_key:
            print(indent + '└──', end=' ')
            new_indent = indent + '   '
        else:
            print(indent + '├──', end=' ')
            new_indent = indent + '│  '
        
        if isinstance(value, dict):
            print(key)
            print_tree(value, new_indent)
        else:
            print(key)