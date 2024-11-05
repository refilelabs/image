import { supportedFormats } from '#image/utils/file_types'

export const useImageFormats = () => {
  const imageSourceFormats = supportedFormats.filter(format => format.decoding !== 'No').map(format => format.format)
  const imageTargetFormats = supportedFormats.filter(format => format.encoding !== 'No').map(format => format.format)
  const possibleImageConversions = imageSourceFormats.flatMap(sourceFormat => imageTargetFormats.map(targetFormat => ({ sourceFormat, targetFormat })))
  const allFormats = supportedFormats.map(format => format.format)

  return {
    image: {
      source: imageSourceFormats,
      target: imageTargetFormats,
      all: allFormats,
      conversions: possibleImageConversions,
    },
  }
}
