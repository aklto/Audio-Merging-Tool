# Инструмент для объединения аудио

Это скрипт на языке Rust объединяет два аудиофайла (`audio1.wav` и `audio2.wav`) в один выходной файл (`output.wav`).

## Возможности

- **Чтение WAV файлов**: Читает два входных WAV файла (`audio1.wav` и `audio2.wav`).
- **Обработка аудио**: Ускоряет второй аудиофайл, беря каждый образец (шаг за шагом).
- **Смешивание**: Объединяет образцы двух аудиофайлов, обеспечивая соответствие длины выходного файла самой длинной из двух входных файлов.
- **Запись WAV файла**: Выводит смешанный аудио в `output.wav`.

## Необходимые условия

- Установленный Rust на вашем компьютере.
- Библиотека `hound`. Добавьте её в ваш `Cargo.toml`:
  ```toml
  [dependencies]
  hound = "3.4.0"
  ```

## Как использовать

1. **Подготовьте аудиофайлы**: Убедитесь, что у вас есть два WAV файла, `audio1.wav` и `audio2.wav`, размещенные в каталоге `../files/` относительно места, где запускается программа.

2. **Скомпилируйте и запустите программу**:
   ```bash
   cargo build 
   ```

   ```bash
   cargo run
   ```

3. **Вывод**: После выполнения программы вы найдете `output.wav` в текущем каталоге.

## Дополнительные замечания

- Программа предполагает, что входные аудиофайлы имеют одинаковую частоту дискретизации, количество каналов и битовую глубину.
- По умолчанию второй аудиофайл ускоряется. Если вы хотите изменить шаг, измените метод `step_by` соответственно.
- Обеспечьте правильную обработку ошибок в производственной среде, чтобы обрабатывать случаи, когда файлы могут отсутствовать или быть неправильно отформатированными.
