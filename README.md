# Инструмент для объединения аудио

Эта программа на языке Rust объединяет два аудиофайла (`audio1.wav` и `audio2.wav`) в один выходной файл (`output.wav`). Она использует библиотеку `hound` для чтения и записи файлов WAV.

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

## Пояснение кода

```rust
use hound;
use std::i16;

fn main() {
    // Открыть первый аудиофайл и прочитать его образцы
    let mut reader1 = hound::WavReader::open("../files/audio2.wav").unwrap();
    let spec1 = reader1.spec();
    let samples1: Vec<i16> = reader1.samples::<i16>().map(|s| s.unwrap()).collect();

    // Открыть второй аудиофайл и прочитать его образцы
    let mut reader2 = hound::WavReader::open("../files/audio1.wav").unwrap();
    let spec2 = reader2.spec();
    let samples2: Vec<i16> = reader2.samples::<i16>().map(|s| s.unwrap()).collect();
    // Ускорить второй аудиофайл, беря каждый образец (шаг 1)
    let samples2: Vec<i16> = samples2.iter().step_by(1).copied().collect();

    // Определить длину самого длинного набора образцов
    let length = samples1.len().max(samples2.len());

    // Увеличить второй набор образцов до длины первого
    let mut extended_samples2 = Vec::with_capacity(length);
    extended_samples2.extend_from_slice(&samples2);
    extended_samples2.resize(length, 0);

    // Инициализировать вектор для хранения смешанных образцов
    let mut output_samples: Vec<i16> = Vec::with_capacity(length);

    // Смешать два набора образцов
    for i in 0..length {
        let sample1 = if i < samples1.len() { samples1[i] } else { 0 };
        let sample2 = if i < extended_samples2.len() { extended_samples2[i] } else { 0 };

        // Сложить образцы, предотвращая переполнение
        let mixed_sample = sample1.saturating_add(sample2);
        output_samples.push(mixed_sample);
    }

    // Определить спецификации для выходного WAV файла
    let spec = hound::WavSpec {
        channels: spec1.channels,
        sample_rate: spec1.sample_rate,
        bits_per_sample: spec1.bits_per_sample,
        sample_format: hound::SampleFormat::Int,
    };

    // Создать и записать выходной WAV файл
    let mut writer = hound::WavWriter::create("output.wav", spec).unwrap();
    for sample in output_samples {
        writer.write_sample(sample).unwrap();
    }

    // Завершить работу писателя, чтобы убедиться, что все данные записаны
    writer.finalize().unwrap();
    println!("Аудиофайлы успешно объединены в 'output.wav'");
}
```

## Дополнительные замечания

- Программа предполагает, что входные аудиофайлы имеют одинаковую частоту дискретизации, количество каналов и битовую глубину.
- По умолчанию второй аудиофайл ускоряется. Если вы хотите изменить шаг, измените метод `step_by` соответственно.
- Обеспечьте правильную обработку ошибок в производственной среде, чтобы обрабатывать случаи, когда файлы могут отсутствовать или быть неправильно отформатированными.

Следуя этим инструкциям, вы сможете скомпилировать, запустить и понять основную функциональность инструмента для объединения аудио.