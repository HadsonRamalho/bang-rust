import {
  Sheet,
  SheetTrigger,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@/components/ui/sheet";
import { Card, CardHeader, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { CardSvgIcon } from "./card-svg-icon";
import type { Carta } from "@/interfaces/cards/cards";

interface DetalhesCartaProps {
  carta: Carta
}

export default function DetalhesCarta({ carta }: DetalhesCartaProps) {
  const [tipo, info] = Object.entries(carta)[0];

  return (
    <Sheet>
      <SheetTrigger>
        <Button className="bg-[hsl(var(--primary))] hover:cursor-pointer">
          Ver detalhes
        </Button>
      </SheetTrigger>
      <SheetContent className="bg-gray-200">
        <SheetHeader>
          <SheetTitle>Detalhes da Carta</SheetTitle>
          <SheetDescription>
            <Card className="border-[hsl(var(--primary))]">
              <CardHeader className="flex items-center space-x-0">
                <CardSvgIcon tipo={tipo} size={45} />
                <p>
                  <strong>{tipo}</strong>
                </p>
              </CardHeader>
              <CardContent>
                <p>
                  <strong>Descrição: </strong>
                  {info.descricao}
                </p>
              </CardContent>
            </Card>
          </SheetDescription>
        </SheetHeader>
      </SheetContent>
    </Sheet>
  );
}
