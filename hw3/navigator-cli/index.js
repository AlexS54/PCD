//Load openDSU enviroment
require("../opendsu-sdk/builds/output/openDSU");
const opendsu = require("opendsu");
const resolver = opendsu.loadApi("resolver");
const keyssispace = opendsu.loadApi("keyssi");

const chalk = require("chalk").default;
const readline = require("node:readline");

let keySSI = process.argv[2];

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

const createSsiAndDsu = () => {
  keyssispace.createTemplateSeedSSI(
    "default",
    undefined,
    undefined,
    undefined,
    undefined,
    (err, ssi) => {
      if (err) {
        console.error("Failed to create SSI", err);
        return;
      }
      resolver.createDSU(ssi, undefined, (err, dsuInstance) => {
        if (err) {
          console.error("Failed to create DSU:", err);
          return;
        }

        dsuInstance.getKeySSIAsString((err, keyidentifier) => {
          if (err) throw err;

          keySSI = keyidentifier;
          console.log("Created DSU associated with KeySSI:\n", keyidentifier);

          startCliLoop();
        });
      });
    }
  );
};

const errorCallbackWrapper = (callback) => (err, data) => {
  if (err) {
    console.error(err.originalMessage);
    return prompt();
  }

  callback(data);
};

const promptCreateSSI = () => {
  rl.question(
    "No KeySSI inputted. Would you like to create one? Y/n",
    (answer) => {
      if (!answer || answer.toLowerCase() == "y") createSsiAndDsu();
    }
  );
};

let cwd = "/";

const cwdOrRoot = () => (cwd == "/" ? "" : cwd);

/**
 *
 * @param {string} path
 * @returns
 */
const computeRelativePath = (path) => {
  if (path.startsWith("/")) return path;

  return cwdOrRoot() + "/" + path;
};

const stringifyCwd = () => {
  return chalk.green(`${cwd} $ `);
};

const loadDSU = (callback) => {
  resolver.loadDSU(keySSI, undefined, errorCallbackWrapper(callback));
};

const commitBatchAndResume = (dsu, batchId) => {
  dsu.commitBatch(batchId, (err) => {
    if (err) {
      console.error(err);
    }

    prompt();
  });
};

const cat = (path) => {
  if (!path) {
    console.log("Specify a path!");
    return prompt();
  }

  loadDSU((dsu) => {
    dsu.readFile(
      computeRelativePath(path),
      undefined,
      errorCallbackWrapper((content) => {
        console.log(content.toString());
        prompt();
      })
    );
  });
};

/**
 *
 * @param {string} path
 * @param {string} content
 */
const echo = (path, content) => {
  if (!path || !content) {
    console.log("Usage: echo <path> <content>");
    return prompt();
  }

  loadDSU((dsu) => {
    dsu.safeBeginBatch(
      errorCallbackWrapper((batchId) => {
        dsu.writeFile(
          computeRelativePath(path),
          content,
          errorCallbackWrapper(() => {
            commitBatchAndResume(dsu, batchId);
          })
        );
      })
    );
  });
};

/**
 *
 * @param {string | undefined} path
 */
const ls = (path) => {
  loadDSU((dsu) => {
    const toLog = [];
    const pathToLs = path == null ? cwd : computeRelativePath(path);

    dsu.listFolders(
      pathToLs,
      errorCallbackWrapper((folders) => {
        folders.forEach((folder) => toLog.push(chalk.blue(folder)));

        dsu.listFiles(
          pathToLs,
          { recursive: false },
          errorCallbackWrapper((files) => {
            files.forEach((file) => toLog.push(file));

            console.log(toLog.join(" "));
            prompt();
          })
        );
      })
    );
  });
};

/**
 *
 * @param {string} localPath
 * @param {string} destPath
 */
const put = (localPath, destPath) => {
  if (!localPath || !destPath) {
    console.log("Usage: put <local_path> <destination_path>");
    return prompt();
  }

  loadDSU((dsu) => {
    dsu.safeBeginBatch(
      errorCallbackWrapper((batchId) => {
        dsu.addFile(
          localPath,
          computeRelativePath(destPath),
          errorCallbackWrapper(() => {
            commitBatchAndResume(dsu, batchId);
          })
        );
      })
    );
  });
};

/**
 *
 * @param {string} path
 */
const cd = (path) => {
  path = path || "/";

  cwd = computeRelativePath(path);
  prompt();
};

/**
 *
 * @param {string} folderName
 */
const mkdir = (folderName) => {
  if (folderName == null) {
    console.log("Specify a folder name!");
    return prompt();
  }

  loadDSU((dsu) => {
    dsu.safeBeginBatch(
      errorCallbackWrapper((batchId) => {
        dsu.createFolder(
          computeRelativePath(folderName),
          errorCallbackWrapper(() => {
            commitBatchAndResume(dsu, batchId);
          })
        );
      })
    );
  });
};

/**
 *
 * @param {string} path
 */
const rm = (path) => {
  if (!path) {
    console.log("Specify a path!");
    return prompt();
  }

  loadDSU((dsu) => {
    dsu.safeBeginBatch(
      errorCallbackWrapper((batchId) => {
        dsu.delete(
          computeRelativePath(path),
          errorCallbackWrapper(() => commitBatchAndResume(dsu, batchId))
        );
      })
    );
  });
};

const prompt = () => {
  rl.question(stringifyCwd(), onAnswer);
};

/**
 * @param {string} answer
 */
const onAnswer = (answer) => {
  const args = answer.trim().split(" ");

  switch (args[0]) {
    case "cd":
      cd(args[1]);
      break;
    case "ls":
      ls(args[1]);
      break;
    case "cat":
      cat(args[1]);
      break;
    case "echo":
      echo(args[1], args[2]);
      break;
    case "put":
      put(args[1], args[2]);
      break;
    case "rm":
      rm(args[1]);
      break;
    case "mkdir":
      mkdir(args[1]);
      break;
    case "exit":
      rl.close();
      return;
    default:
      console.log("Unknown command:", args[0]);
      prompt();
      break;
  }
};

const startCliLoop = () => {
  prompt();
};

const loadDsu = () => {
  resolver.loadDSU(keySSI, undefined, (err, dsu) => {
    if (err) {
      console.error("Failed to load DSU", err);
      return;
    }

    startCliLoop();
  });
};

if (keySSI == null) {
  promptCreateSSI();
} else {
  loadDsu();
}
