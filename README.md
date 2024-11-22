# 🔧 Fixr - Taşınabilir Disk Yönetim Aracı

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)

Fixr, Windows sistemlerde taşınabilir diskleri yönetmek ve onarım işlemlerini kolaylaştırmak için geliştirilmiş, komut satırı tabanlı bir araçtır.

## ✨ Özellikler

- 📝 Sistemdeki tüm taşınabilir diskleri listeleme
- 🔍 Disk detaylarını görüntüleme (boyut, kullanım oranı, vb.)
- 🛠️ Disk onarımı gerçekleştirme
- 🎨 Renkli ve kullanıcı dostu arayüz

## 🚀 Kurulum

```bash
# Repository'yi klonlayın
git clone https://github.com/kullaniciadi/fixr.git

# Proje dizinine gidin
cd fixr

# Uygulamayı derleyin
cargo build --release

# Çalıştırılabilir dosya target/release dizininde oluşturulacaktır
```

## 📖 Kullanım

### Taşınabilir Diskleri Listeleme

```bash
# Basit liste
fixr list

# Detaylı liste (boyut bilgileri ile)
fixr list --verbose
```

### Disk Bilgilerini Görüntüleme

```bash
# F: sürücüsünün bilgilerini göster
fixr info F:
```

### Disk Onarımı

```bash
# Temel onarım
fixr fix F:

# Zorla onarım (dikkatli kullanın!)
fixr fix F: --force
```

## 🔍 Komut Detayları

### `list` Komutu
- Sistemdeki tüm taşınabilir diskleri listeler
- `--verbose` parametresi ile detaylı bilgileri gösterir

### `info` Komutu
- Belirtilen diskin detaylı bilgilerini gösterir:
  - Toplam alan
  - Kullanılan alan
  - Boş alan
  - Kullanım yüzdesi

### `fix` Komutu
- Disk onarımı gerçekleştirir
- `--force` parametresi ile zorla onarım yapabilir
- Windows'un `chkdsk` aracını kullanır

## ⚠️ Önemli Notlar

1. Onarım işlemi öncesi önemli verilerinizi yedekleyin
2. `--force` parametresini dikkatli kullanın
3. Programı yönetici (Administrator) olarak çalıştırmanız gerekebilir

## 🛠️ Geliştirme

### Gereksinimler

- Rust 1.75 veya üzeri
- Windows işletim sistemi
- Cargo ve ilgili araçlar

### Bağımlılıklar

- clap: Komut satırı argüman işleme
- colored: Terminal renklendirme
- windows: Windows API entegrasyonu

## 📝 Lisans

Bu proje MIT lisansı altında lisanslanmıştır. Detaylar için [LICENSE](LICENSE) dosyasına bakınız.

## 🤝 Katkıda Bulunma

1. Bu repository'yi fork edin
2. Yeni bir branch oluşturun (`git checkout -b feature/yeniOzellik`)
3. Değişikliklerinizi commit edin (`git commit -am 'Yeni özellik eklendi'`)
4. Branch'inizi push edin (`git push origin feature/yeniOzellik`)
5. Bir Pull Request oluşturun

## 📞 İletişim

Sorularınız veya önerileriniz için lütfen GitHub üzerinden issue açın.
