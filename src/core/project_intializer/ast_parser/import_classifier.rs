pub struct ImportClassifier {
    stdlib_modules: Vec<String>
}

impl ImportClassifier {
    pub fn new(&self) -> Self {
        let stdlib_modules: Vec<String> = [
            "__phello__", "_pyrepl", "asyncio", "collections", "compression",
            "concurrent", "ctypes", "curses", "dbm", "email", "encodings",
            "ensurepip", "html", "http", "idlelib", "importlib", "json",
            "logging", "multiprocessing", "pathlib", "profiling", "pydoc_data",
            "re", "site-packages", "sqlite3", "string", "sysconfig", "test",
            "tkinter", "tomllib", "turtledemo", "unittest", "urllib", "venv",
            "wsgiref", "xml", "xmlrpc", "zipfile", "zoneinfo", "__future__", "__hello__",
            "_aix_support", "_android_support", "_apple_support", "_ast_unparse",
            "_collections_abc", "_colorize", "_compat_pickle", "_ios_support", "_markupbase",
            "_opcode_metadata", "_osx_support", "_py_abc", "_py_warnings", "_pydatetime",
            "_pydecimal", "_pyio", "_pylong", "_sitebuiltins", "_strptime", "_threading_local",
            "_weakrefset", "abc", "annotationlib", "antigravity", "argparse", "ast", "base64",
            "bdb", "bisect", "bz2", "cProfile", "calendar", "cmd", "code", "codecs",
            "codeop", "colorsys", "compileall", "configparser", "contextlib", "contextvars",
            "copy", "copyreg", "csv", "dataclasses", "datetime", "decimal", "difflib",
            "dis", "doctest", "enum", "filecmp", "fileinput", "fnmatch", "fractions",
            "ftplib", "functools", "genericpath", "getopt", "getpass", "gettext", "glob",
            "graphlib", "gzip", "hashlib", "heapq", "hmac", "imaplib", "inspect", "io",
            "ipaddress", "keyword", "linecache", "locale", "lzma", "mailbox", "mimetypes",
            "modulefinder", "netrc", "ntpath", "nturl2path", "numbers", "opcode", "operator",
            "optparse", "os", "pdb", "pickle", "pickletools", "pkgutil", "platform", "plistlib",
            "poplib", "posixpath", "pprint", "profile", "pstats", "pty", "py_compile", "pyclbr",
            "pydoc", "queue", "quopri", "random", "reprlib", "rlcompleter", "runpy", "sched",
            "secrets", "selectors", "shelve", "shlex", "shutil", "signal", "site", "smtplib",
            "socket", "socketserver", "ssl", "stat", "statistics", "stringprep", "struct",
            "subprocess", "symtable", "tabnanny", "tarfile", "tempfile", "textwrap", "this",
            "threading", "timeit", "token", "tokenize", "trace", "traceback", "tracemalloc",
            "tty", "turtle", "types", "typing", "uuid", "warnings", "wave", "weakref",
            "webbrowser", "zipapp", "zipimport"
        ].iter().map(|s| s.to_string()).collect();

        Self {stdlib_modules}
    }

    pub fn is_eligible(&self, import: String) -> bool {
        println!("hello");
        return true;
    }


}