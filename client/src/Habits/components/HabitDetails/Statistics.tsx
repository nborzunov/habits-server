import { Flex, Icon, Text } from '@chakra-ui/react';
import dayjs from 'dayjs';
import { IconType } from 'react-icons/lib';
import Icons from '~/common/helpers/Icons';
import { formatDays } from '~/common/utils/formatDays';

const Statistics = ({
    icon,
    title,
    value,
    footerValue,
    type,
    startDate,
}: {
    title: string;
    value: number;
    icon?: IconType;
    type: 'increase' | 'decrease' | 'streak' | 'none';
    footerValue?: number;
    startDate?: Date;
}) => {
    return (
        <Flex justifyContent='space-between' w='100%'>
            <Flex alignItems='center'>
                {type === 'streak' && <Icon color='red.500' as={Icons.Streak} h='8' w='8' mr='2' />}
                <Flex flexDir='column' justifyContent='center' h='100%'>
                    <Flex alignItems='center'>
                        {type !== 'streak' && icon && <Icon as={icon} mr='1' />}
                        <Text fontWeight='normal' letterSpacing='wide' fontSize='md'>
                            {title}
                        </Text>
                    </Flex>
                    <Text fontSize='md' fontWeight='bold' pl={icon ? '1' : '0'} pb={'1'}>
                        {formatDays(value)}
                    </Text>
                    {footerValue ? (
                        <>
                            {type === 'increase' && (
                                <Flex alignItems='center' color='green.500' mb='2'>
                                    <Icon as={Icons.ArrowTop} />
                                    <Text color='inherit' fontSize='12px' fontWeight='semibold'>
                                        {formatDays(footerValue)}
                                    </Text>
                                </Flex>
                            )}
                            {type === 'decrease' && (
                                <Flex alignItems='center' color='red.500' mb='2'>
                                    <Icon as={Icons.ArrowBottom} />
                                    <Text color='inherit' fontSize='12px' fontWeight='semibold'>
                                        {formatDays(footerValue)}
                                    </Text>
                                </Flex>
                            )}
                        </>
                    ) : (
                        <Text color='gray.500'>---</Text>
                    )}
                </Flex>
            </Flex>

            {type === 'streak' && startDate && (
                <Flex alignItems='center'>
                    <Text
                        textTransform='uppercase'
                        fontSize='12px'
                        fontWeight='semibold'
                        color='gray.500'
                        bg='gray.100'
                        borderRadius='8'
                        p='2'
                    >
                        From {dayjs(startDate).format('MMM DD, YYYY')}
                    </Text>
                </Flex>
            )}
        </Flex>
    );
};

export default Statistics;
