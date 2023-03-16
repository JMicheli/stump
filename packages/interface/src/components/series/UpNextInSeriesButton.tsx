import { useUpNextInSeries } from '@stump/client'
import { ButtonOrLink } from '@stump/components'

type Props = {
	seriesId: string
	title?: string
}

export default function UpNextInSeriesButton({ seriesId, title, ...props }: Props) {
	const { media, isLoading } = useUpNextInSeries(seriesId)

	// TODO: Change this once Stump supports epub progress tracking.
	if (media?.extension === 'epub') {
		return null
	}

	return (
		<ButtonOrLink
			variant="primary"
			disabled={!isLoading && !media}
			href={`/books/${media?.id}/pages/${media?.current_page || 1}`}
			title={`Continue reading ${media?.name || 'from where you left off'}`}
			{...props}
		>
			{title || 'Continue Reading'}
		</ButtonOrLink>
	)
}
