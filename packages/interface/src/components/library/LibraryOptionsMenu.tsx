import { queryClient, useScanLibrary, useUserStore } from '@stump/client'
import { DropdownMenu, IconButton } from '@stump/components'
import type { Library } from '@stump/types'
import { Edit, FileScan, FolderSearch2, MoreVertical, Trash } from 'lucide-react'
import { useNavigate } from 'react-router-dom'

// import DeleteLibraryModal from './DeleteLibraryModal'
// import EditLibraryModal from './EditLibraryModal'

interface Props {
	library: Library
}

export default function LibraryOptionsMenu({ library }: Props) {
	const navigate = useNavigate()

	const user = useUserStore((store) => store.user)

	const { scanAsync } = useScanLibrary()

	function handleScan() {
		// extra protection, should not be possible to reach this.
		if (user?.role !== 'SERVER_OWNER') {
			throw new Error('You do not have permission to scan libraries.')
		}

		// The UI will receive updates from SSE in fractions of ms lol and it can get bogged down.
		// So, add a slight delay so the close animation of the menu can finish cleanly.
		setTimeout(async () => {
			await scanAsync(library.id)
			await queryClient.invalidateQueries(['getJobReports'])
		}, 50)
	}

	return (
		<DropdownMenu
			trigger={
				<IconButton variant="ghost" rounded="full" size="xs">
					<MoreVertical className="h-4 w-4" />
				</IconButton>
			}
			groups={[
				{
					items: [
						{
							label: 'Scan Library',
							leftIcon: <FileScan className="mr-2 h-4 w-4" />,
							subItems: [
								{
									label: 'Batched Scan',
								},
								{
									label: 'Synchronous Scan',
								},
							],
						},
						{
							label: 'File Explorer',
							leftIcon: <FolderSearch2 className="mr-2 h-4 w-4" />,
						},
					],
				},
				{
					items: [
						{
							label: 'Edit Library',
							leftIcon: <Edit className="mr-2 h-4 w-4" />,
						},
						{
							label: 'Delete Library',
							leftIcon: <Trash className="mr-2 h-4 w-4" />,
						},
					],
				},
			]}
			align="start"
		/>
	)

	// // FIXME: so, disabled on the MenuItem doesn't seem to actually work... how cute.
	// return (
	// 	// TODO: https://chakra-ui.com/docs/theming/customize-theme#customizing-component-styles
	// 	<div>
	// 		<Menu size="sm">
	// 			<MenuButton
	// 				disabled={user?.role !== 'SERVER_OWNER'}
	// 				hidden={user?.role !== 'SERVER_OWNER'}
	// 				p={1}
	// 				rounded="full"
	// 				_hover={{ _dark: { bg: 'gray.700' }, bg: 'gray.200' }}
	// 				_focus={{
	// 					boxShadow: '0 0 0 2px rgba(196, 130, 89, 0.6);',
	// 				}}
	// 				_active={{
	// 					boxShadow: '0 0 0 2px rgba(196, 130, 89, 0.6);',
	// 				}}
	// 			>
	// 				{/* <DotsThreeVertical size={'1.25rem'} /> */}
	// 			</MenuButton>

	// 			<MenuList>
	// 				{/* TODO: scanMode */}
	// 				<MenuItem
	// 					disabled={user?.role !== 'SERVER_OWNER'}
	// 					// icon={<ArrowsClockwise size={'1rem'} />}
	// 					onClick={handleScan}
	// 				>
	// 					Scan
	// 				</MenuItem>
	// 				<EditLibraryModal library={library} disabled={user?.role !== 'SERVER_OWNER'} />
	// 				<MenuItem
	// 					// icon={<Binoculars size={'1rem'} />}
	// 					onClick={() => navigate(`libraries/${library.id}/explorer`)}
	// 				>
	// 					File Explorer
	// 				</MenuItem>
	// 				<MenuDivider />
	// 				<DeleteLibraryModal library={library} disabled={user?.role !== 'SERVER_OWNER'} />
	// 			</MenuList>
	// 		</Menu>
	// 	</div>
	// )
}
