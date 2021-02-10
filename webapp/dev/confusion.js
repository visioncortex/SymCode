function htmlCollectionToArray(htmlCollection) {
    return [].slice.call(htmlCollection);
}

function extractUsefulInnerText(tag) {
    return htmlCollectionToArray(document.getElementsByTagName(tag))
        .map(element => element.innerText) // extract the innerText
        .filter(text => !(/^\d/.test(text))) // filter out the binary representations
        .map(text => extractUsefulText(text)); // Remove unnecessary parts of the text
}

// Remove unwanted parts recursively
function extractUsefulText(text) {
    if (text.startsWith("(")) {
        return extractUsefulText(text.substring("(".length));
    } else if (text.startsWith("Some")) {
        return extractUsefulText(text.substring("Some".length));
    } else if (text.startsWith("[")) {
        return extractUsefulText(text.substring("[".length));
    } else if (text.endsWith(")")) {
        return extractUsefulText(text.substring(0, text.length - ")".length));
    } else if (text.endsWith("]")) {
        return extractUsefulText(text.substring(0, text.length - "]".length));
    } else {
        return text;
    }
}

function drawConfusionMatrixToHtmlTable(tableId, confusionMatrix) {
    const table = document.getElementById(tableId);
    if (!table || table.tagName.localeCompare("TABLE") != 0) {
        console.log("No <table> element found with id " + tableId);
        return;
    }
    let html = [];

    for (let i = 0; i < confusionMatrix.length; ++i) {
        html.push(`<tr>`);
        confusionMatrix[i].forEach((text, j) => {
            if (i == 0 || j == 0) {
                html.push(`<th>${text}</th>`);
            } else if (i == j) {
                html.push(`<td>-</td>`)
            } else {
                html.push(
                    `<td><div class="tooltip">${text}
                    <span class="tooltiptext">${confusionMatrix[i][0]}\\${confusionMatrix[0][j]}</span></div></td>`
                );
            }
        });
        html.push(`</tr>`);
    }

    table.innerHTML = html.join("");
}

export function calculateConfusionMatrix(targetTableId) {
    let mistakens = extractUsefulInnerText("del");
    let groundTruths = extractUsefulInnerText("ins");
    if (mistakens.length != groundTruths.length) {
        console.log("Error in calculating confusion matrix: lengths of mistakens and groundTruths do not agree.");
        return;
    }
    // groundTruths[i] is mistakenly recognized as mistakens[i]

    // Obtain the set of labels
    let labelSet = new Set([...mistakens, ...groundTruths]);
    const labelArray = [...labelSet].sort((a, b) => {
        // Put None to the last
        if (a.localeCompare("None") == 0) {
            return 1;
        } else if (b.localeCompare("None") == 0) {
            return -1;
        }
        return a.localeCompare(b);
    });

    // Count the confusions
    let confusions = {}; // Ground-truth to Object of confusions with each mistaken
    for (let i = 0; i < groundTruths.length; ++i) {
        const groundTruth = groundTruths[i];
        const mistaken = mistakens[i];
        if (!confusions[groundTruth]) { // haven't seen this groundTruth yet
            confusions[groundTruth] = {};
        }
        if (!confusions[groundTruth][mistaken]) { // haven't seen this confusion for this groundTruth yet
            confusions[groundTruth][mistaken] = 1;
        } else {
            confusions[groundTruth][mistaken] += 1;
        }
    }

    if (labelArray.length == 0) {
        return;
    }

    // Construct the confusion matrix

    // Allocate a 2d array, reserving 1 row and 1 column for the labels
    let confusionMatrix = Array.from(Array(labelArray.length+1), () => new Array(labelArray.length+1));

    // Initialize (origin at top-left corner)
    confusionMatrix[0][0] = "Ground-truth \\ Mistaken";
    for (let i = 1; i < confusionMatrix.length; ++i) {
        confusionMatrix[i][0] = labelArray[i-1];
        confusionMatrix[0][i] = labelArray[i-1];
    }

    // Fill in the confusions
    for (let i = 1; i < confusionMatrix.length; ++i) {
        for (let j = 1; j < confusionMatrix[i].length; ++j) {
            try {
                const numConfusions = confusions[labelArray[i-1]][labelArray[j-1]];
                confusionMatrix[i][j] = numConfusions? numConfusions : 0;
            } catch (e) {
                confusionMatrix[i][j] = 0;
            }
        }
    }

    drawConfusionMatrixToHtmlTable(targetTableId, confusionMatrix);
}