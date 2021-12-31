# DEPRECATED
# butcher

### Vim plugin to quickly parse strings into arrays.
### It is painful to write arrays in any programming language, so butcher makes it easy for you.


## Installation
With vim-plug:
```
Plug "zim0369/butcher"
```
Add this to your vimrc:
```
noremap <leader>n :'<,'>. !xargs $HOME/.vim/plugged/butcher/nums<CR>
noremap <leader>s :'<,'>. !xargs $HOME/.vim/plugged/butcher/string<CR>
```


## Usage
Suppose you have this line in your program that you want to turn into an array of strings.
```
Turn this into an array of strings
```
Now visually select the entire line and press `<leader>s`.

The above line gets replaced with this beautiful array :
```
["Turn", "this", "into", "an", "array", "of", "strings"]
```

Array of characters?
Suppose you have this word in your program that you want to turn into an array of characters.
```
Characters
```
Now visually select the entire line and press `<leader>s`.

The above line gets replaced with this beautiful array :
```
['C', 'h', 'a', 'r', 'a', 'c', 't', 'e', 'r', 's']
```

Array of numbers?
Suppose you have this line in your program that you want to turn into an array of numbers.
```
1 2 4 5 4 5 5
```
Now visually select the entire line and press `<leader>n`.

The above line gets replaced with this beautiful array :
```
[1, 2, 4, 5, 4, 5, 5]
```

You can go ahead and change the keybindings.
