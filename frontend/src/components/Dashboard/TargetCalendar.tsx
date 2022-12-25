import { Box, Flex, Grid, GridItem, Text, Tooltip, useTheme } from '@chakra-ui/react';
import dayjs from 'dayjs';
import getLoop from '~/utils/getLoop';

const Cell = ({
    dayId,
    monthId,
    daysInMonth,
    size,
}: {
    dayId: number;
    monthId: number;
    daysInMonth: number;
    size: number;
}) => {
    const sizePx = `${size}px`;
    const theme = useTheme();

    if (dayId < 0 || dayId >= daysInMonth) {
        return <Box key={'empty' + monthId + dayId} width={sizePx} height={sizePx} />;
    }

    const filled = Math.random() > 0.3;

    const green = theme.colors.green;

    const greenValues = Object.values(green);

    const randomGreen = greenValues[Math.floor(Math.random() * greenValues.length)] as string;

    return (
        <Box key={monthId + dayId} cursor='pointer'>
            <Tooltip label={dayjs(`2022-${monthId + 1}-${dayId + 1}`).format('D MMMM YYYY')}>
                <Box width={sizePx} height={sizePx} bg={filled ? randomGreen : 'gray.300'}></Box>
            </Tooltip>
        </Box>
    );
};

const Month = ({ monthId, size }: { monthId: number; size: number }) => {
    const daysInMonth = dayjs(`2022-${monthId + 1}-1`).daysInMonth();
    const firstDay = dayjs(`2022-${monthId + 1}-1`).day();
    const columns = Math.ceil((firstDay + daysInMonth) / 7);

    const gaps = [2, 6];
    const cellSizes = [9, 14];
    const gap = gaps[size];
    const cellSize = cellSizes[size];

    const month = dayjs(`2022-${monthId + 1}-1`).format(size === 1 ? 'MMMM' : 'MMM');
    return (
        <Box p='1'>
            <Text pb='1' textAlign='center' fontWeight='bold'>
                {month}
            </Text>
            <Grid templateColumns={`repeat(${columns}, ${cellSize}px)`} gap={`${gap}px`}>
                {getLoop(columns).map((columnId) => (
                    <GridItem key={monthId + columnId}>
                        <Grid templateRows={`repeat(7, ${cellSize}px)`} gap={`${gap}px`}>
                            {getLoop(7).map((rowId) => (
                                <Cell
                                    key={monthId + columnId + rowId}
                                    monthId={monthId}
                                    dayId={columnId * 7 + rowId - firstDay}
                                    daysInMonth={daysInMonth}
                                    size={cellSize}
                                />
                            ))}
                        </Grid>
                    </GridItem>
                ))}
            </Grid>
        </Box>
    );
};
const TargetCalendar = ({ size }: { size?: 'sm' | 'md' } = { size: 'md' }) => {
    return (
        <Box>
            <Flex p='2'>
                {getLoop(12).map((i) => (
                    <Month key={i} monthId={i} size={size === 'sm' ? 0 : 1} />
                ))}
            </Flex>
        </Box>
    );
};

export default TargetCalendar;
