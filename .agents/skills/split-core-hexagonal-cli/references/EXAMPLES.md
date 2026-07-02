# CLI Architecture Examples

Pseudocode examples for Split-Core Hexagonal CLI Architecture.

## Domain entity

```
class User:
  private id: Optional<Identifier>
  private email: EmailValue
  private firstName: StringValue
  private lastName: StringValue

  constructor(params):
    this.email = new EmailValue(params.email)
    this.firstName = new StringValue(params.firstName)
    this.lastName = new StringValue(params.lastName)

  getId(): Identifier:
    if this.id is empty: throw Error('ID_NOT_DEFINED')
    return this.id

  setId(id: Identifier): void:
    this.id = id
```

## Use case contract (domain layer)

```
// core/domain/usecases/ImportUsersUseCase
type ImportUsersUseCaseInput = {
  filePath: string
  content?: string
  dryRun: boolean
  format: 'csv' | 'json'
}

type ImportUsersUseCaseOutput = {
  imported: number
  skipped: number
  dryRun?: boolean
}
```

## Use case implementation

```
class ImportUsersUseCase:
  constructor({ userRepository, fileSystem }):

  async execute(port: ImportUsersUseCaseInput):
    if port.dryRun:
      return { imported: 0, skipped: 0, dryRun: true }

    raw = port.content ?? await fileSystem.readFile(port.filePath)
    rows = parseByFormat(raw, port.format)

    if rows.isEmpty:
      throw Error('NO_ROWS_FOUND')

    // ... validate, create User entities, save
    saved = await userRepository.saveMany(users)
    return { imported: saved.length, skipped: rows.length - saved.length }
```

## Repository port

```
interface UserRepository:
  findByEmails(emails: string[]): Promise<Map<string, User>>
  saveMany(users: User[]): Promise<User[]>
```

## Infrastructure adapter

```
class UserDatabaseRepository implements UserRepository:
  constructor(db: DatabaseConnection):

  async saveMany(users: User[]):
    result = await db.insertMany(USER_TABLE, users.map(toRow))
    if result.failed: throw Error('USER_NOT_SAVED')
    users.forEach((u, i) => u.setId(result.ids[i]))
    return users
```

## I/O ports (domain)

```
interface OutputWriter:
  writeLine(text: string): void
  write(data: string): void

interface InputReader:
  readAll(): Promise<string>

interface PromptService:
  confirm(message: string): Promise<boolean>
  input(message: string, hidden?: boolean): Promise<string>

interface FileSystemPort:
  readFile(path: string): Promise<string>
  writeFile(path: string, content: string): Promise<void>
  exists(path: string): Promise<boolean>
```

## Command handler

```
class ImportUsersCommand:
  constructor({ importUsersUseCase, inputReader, jsonFormatter, humanFormatter }):

  async run(args, global):
    port = {
      filePath: args.file ?? '-',
      dryRun: global.dryRun,
      format: args.format ?? 'csv',
    }
    if port.filePath == '-':
      port.content = await inputReader.readAll()

    result = await importUsersUseCase.execute(port)
    formatter = global.output == 'json' ? jsonFormatter : humanFormatter
    formatter.writeImportResult(result)
```

## Formatter

```
class JsonFormatter:
  constructor(outputWriter: OutputWriter):

  writeImportResult(result: ImportUsersUseCaseOutput):
    outputWriter.writeLine(serializeJson(result))
```

## Exit code map

```
function resolveExitCode(error: Error): number:
  code = error.message
  if code == 'INVALID_ARGUMENT': return 2
  if ERROR_CATALOG[code]: return ERROR_CATALOG[code].exitCode
  return 1
```

## Bootstrap index

```
async function main():
  globalOptions, command, args = parseArgv(process.argv)
  await di.initialize(globalOptions)
  try:
    await command.run(args, globalOptions)
    exit(0)
  catch (e):
    writeError(stderr, e, globalOptions)
    exit(resolveExitCode(e))
  finally:
    await di.shutdown()
```

## Composition root (di)

```
// infrastructure/di
userRepository = new UserDatabaseRepository(db)
fileSystem = new LocalFileSystemAdapter()
importUsersUseCase = new ImportUsersUseCase({ userRepository, fileSystem })
importUsersCommand = new ImportUsersCommand({
  importUsersUseCase,
  inputReader: new StdioReader(),
  jsonFormatter: new JsonFormatter(stdoutWriter),
  humanFormatter: new HumanFormatter(stdoutWriter),
})
export { importUsersCommand, importUsersUseCase }
```

## Test — use case

```
describe('ImportUsersUseCase'):
  useCase = new ImportUsersUseCase({
    userRepository: userRepositoryMock,
    fileSystem: fileSystemMock,
  })

  it('skips save when dryRun'):
    result = await useCase.execute({ filePath: '/x', dryRun: true, format: 'csv' })
    expect(result.dryRun).toBe(true)
    expect(userRepositoryMock.saveMany).not.toHaveBeenCalled()

  it('throws when no rows'):
    fileSystemMock.readFile.returns('')
    await expect(useCase.execute({ filePath: '/x', dryRun: false, format: 'csv' }))
      .rejects.toThrow('NO_ROWS_FOUND')
```

## Test — command handler

```
it('maps flags to ImportUsersUseCaseInput'):
  await command.run({ file: '/data.csv', format: 'csv' }, { dryRun: false, output: 'json' })
  expect(importUsersUseCase.execute).toHaveBeenCalledWith({
    filePath: '/data.csv',
    dryRun: false,
    format: 'csv',
  })

it('reads stdin when file is dash'):
  inputReaderMock.readAll.returns('a@b.com\n')
  await command.run({ file: '-' }, { dryRun: false, output: 'human' })
  expect(importUsersUseCase.execute).toHaveBeenCalledWith(
    expect.objectContaining({ content: 'a@b.com\n' })
  )
```
