# https://github.com/BurntSushi/ripgrep

echo 'part 1:'
cat input | ./rg '(.*[aeiou]){3}' | ./rg --pcre2 '([a-z])\1' | ./rg -v -c 'ab|cd|pq|xy'
echo 'part 2:'
cat input | ./rg --pcre2 '(\w)[a-z]\1' | ./rg -c --pcre2 '(\w)(\w).*\1\2'
