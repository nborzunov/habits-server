import { Box, Icon, IconButton } from '@chakra-ui/react';
import Icons from '~/common/helpers/Icons';

const Widget = ({
    showCross,
    remove,
    children,
}: {
    showCross: boolean;
    remove: () => void;
    children: any;
}) => {
    return (
        <Box
            borderRadius='xl'
            borderColor='gray.200'
            borderWidth='2px'
            p='2'
            display='flex'
            justifyContent='center'
            height={'100%'}
            position={'relative'}
        >
            {showCross && (
                <IconButton
                    icon={<Icon as={Icons.Cross} />}
                    aria-label={'remove widget'}
                    top={'0'}
                    position={'absolute'}
                    right={'0'}
                    size={'sm'}
                    variant={'ghost'}
                    onClick={remove}
                    onDragStart={(e) => e.stopPropagation()}
                />
            )}
            {children}
        </Box>
    );
};

export default Widget;
