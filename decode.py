s = u'\xd5\xd2\xb5\xbd MSIL .netmodule \xbb\xf2\xca\xb9\xd3\xc3 /GL \xb1\xe0\xd2\xeb\xb5\xc4\xc4\xa3\xbf\xe9\xa3\xbb\xd5\xfd\xd4\xda\xca\xb9\xd3\xc3 /LTCG\xd6\xd8\xd0\xc2\xc6\xf4\xb6\xaf\xc1\xb4\xbd\xd3\xa3\xbb\xbd\xab /LTCG \xcc\xed\xbc\xd3\xb5\xbd\xc1\xb4\xbd\xd3\xc3\xfc\xc1\xee\xd0\xd0\xd2\xd4\xb8\xc4\xbd\xf8\xc1\xb4\xbd\xd3\xc6\xf7\xd0\xd4\xc4\xdc\r\nfatal error C1007: \xce\xde\xb7\xa8\xca\xb6\xb1\xf0\xb5\xc4\xb1\xea\xd6\xbe\xa1\xb0-Ot\xa1\xb1(\xd4\xda\xa1\xb0p2\xa1\xb1\xd6\xd0)\r\nLINK : fatal error LNK1257: \xb4\xfa\xc2\xeb\xc9\xfa\xb3\xc9\xca\xa7\xb0\xdc\r\n'
a = s.encode('unicode_escape').decode('string_escape')
print a