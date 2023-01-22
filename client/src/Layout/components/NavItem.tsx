import { Flex, Icon } from '@chakra-ui/react';
import { IconType } from 'react-icons/lib';

interface NavItemProps {
    icon?: IconType;
}

const NavItem = (props: React.PropsWithChildren<NavItemProps>) => {
    const { icon, children, ...rest } = props;
    return (
        <Flex
            align='center'
            px='4'
            mx='4'
            rounded='md'
            py='3'
            cursor='pointer'
            color='gray.600'
            _hover={{
                bg: 'purple.300',
                color: 'whiteAlpha.900',
            }}
            role='group'
            fontWeight='semibold'
            transition='.15s ease'
            {...rest}
        >
            {icon && <Icon mr='2' boxSize='4' as={icon} />}
            {children}
        </Flex>
    );
};

export default NavItem;
