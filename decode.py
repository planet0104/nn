s = u' note: Non-UTF-8 output: LINK : fatal error LNK1181: \xce\xde\xb7\xa8\xb4\xf2\xbf\xaa\xca\xe4\xc8\xeb\xce\xc4\xbc\xfe\xa1\xb0lept.lib\xa1\xb1\r\n'
a = s.encode('unicode_escape').decode('string_escape')
print a