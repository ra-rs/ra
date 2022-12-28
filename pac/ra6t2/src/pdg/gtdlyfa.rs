#[doc = "Register `GTDLYF%sA` reader"]
pub struct R(crate::R<GTDLYFA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTDLYFA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTDLYFA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTDLYFA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTDLYF%sA` writer"]
pub struct W(crate::W<GTDLYFA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTDLYFA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GTDLYFA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTDLYFA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY` reader - GTIOCnA Output Falling Edge Delay Setting"]
pub type DLY_R = crate::FieldReader<u8, DLY_A>;
#[doc = "GTIOCnA Output Falling Edge Delay Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLY_A {
    #[doc = "0: Delay on falling edges is not applied"]
    _0X00 = 0,
    #[doc = "1: Delay of 1/32 times GTCLK period applied"]
    _0X01 = 1,
    #[doc = "2: Delay of 2/32 times GTCLK period applied"]
    _0X02 = 2,
    #[doc = "3: Delay of 3/32 times GTCLK period applied"]
    _0X03 = 3,
    #[doc = "4: Delay of 4/32 times GTCLK period applied"]
    _0X04 = 4,
    #[doc = "5: Delay of 5/32 times GTCLK period applied"]
    _0X05 = 5,
    #[doc = "6: Delay of 6/32 times GTCLK period applied"]
    _0X06 = 6,
    #[doc = "7: Delay of 7/32 times GTCLK period applied"]
    _0X07 = 7,
    #[doc = "8: Delay of 8/32 times GTCLK period applied"]
    _0X08 = 8,
    #[doc = "9: Delay of 9/32 times GTCLK period applied"]
    _0X09 = 9,
    #[doc = "10: Delay of 10/32 times GTCLK period applied"]
    _0X0A = 10,
    #[doc = "11: Delay of 11/32 times GTCLK period applied"]
    _0X0B = 11,
    #[doc = "12: Delay of 12/32 times GTCLK period applied"]
    _0X0C = 12,
    #[doc = "13: Delay of 13/32 times GTCLK period applied"]
    _0X0D = 13,
    #[doc = "14: Delay of 14/32 times GTCLK period applied"]
    _0X0E = 14,
    #[doc = "15: Delay of 15/32 times GTCLK period applied"]
    _0X0F = 15,
    #[doc = "16: Delay of 16/32 times GTCLK period applied"]
    _0X10 = 16,
    #[doc = "17: Delay of 17/32 times GTCLK period applied"]
    _0X11 = 17,
    #[doc = "18: Delay of 18/32 times GTCLK period applied"]
    _0X12 = 18,
    #[doc = "19: Delay of 19/32 times GTCLK period applied"]
    _0X13 = 19,
    #[doc = "20: Delay of 20/32 times GTCLK period applied"]
    _0X14 = 20,
    #[doc = "21: Delay of 21/32 times GTCLK period applied"]
    _0X15 = 21,
    #[doc = "22: Delay of 22/32 times GTCLK period applied"]
    _0X16 = 22,
    #[doc = "23: Delay of 23/32 times GTCLK period applied"]
    _0X17 = 23,
    #[doc = "24: Delay of 24/32 times GTCLK period applied"]
    _0X18 = 24,
    #[doc = "25: Delay of 25/32 times GTCLK period applied"]
    _0X19 = 25,
    #[doc = "26: Delay of 26/32 times GTCLK period applied"]
    _0X1A = 26,
    #[doc = "27: Delay of 27/32 times GTCLK period applied"]
    _0X1B = 27,
    #[doc = "28: Delay of 28/32 times GTCLK period applied"]
    _0X1C = 28,
    #[doc = "29: Delay of 29/32 times GTCLK period applied"]
    _0X1D = 29,
    #[doc = "30: Delay of 30/32 times GTCLK period applied"]
    _0X1E = 30,
    #[doc = "31: Delay of 31/32 times GTCLK period applied"]
    _0X1F = 31,
}
impl From<DLY_A> for u8 {
    #[inline(always)]
    fn from(variant: DLY_A) -> Self {
        variant as _
    }
}
impl DLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLY_A {
        match self.bits {
            0 => DLY_A::_0X00,
            1 => DLY_A::_0X01,
            2 => DLY_A::_0X02,
            3 => DLY_A::_0X03,
            4 => DLY_A::_0X04,
            5 => DLY_A::_0X05,
            6 => DLY_A::_0X06,
            7 => DLY_A::_0X07,
            8 => DLY_A::_0X08,
            9 => DLY_A::_0X09,
            10 => DLY_A::_0X0A,
            11 => DLY_A::_0X0B,
            12 => DLY_A::_0X0C,
            13 => DLY_A::_0X0D,
            14 => DLY_A::_0X0E,
            15 => DLY_A::_0X0F,
            16 => DLY_A::_0X10,
            17 => DLY_A::_0X11,
            18 => DLY_A::_0X12,
            19 => DLY_A::_0X13,
            20 => DLY_A::_0X14,
            21 => DLY_A::_0X15,
            22 => DLY_A::_0X16,
            23 => DLY_A::_0X17,
            24 => DLY_A::_0X18,
            25 => DLY_A::_0X19,
            26 => DLY_A::_0X1A,
            27 => DLY_A::_0X1B,
            28 => DLY_A::_0X1C,
            29 => DLY_A::_0X1D,
            30 => DLY_A::_0X1E,
            31 => DLY_A::_0X1F,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0X00`"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == DLY_A::_0X00
    }
    #[doc = "Checks if the value of the field is `_0X01`"]
    #[inline(always)]
    pub fn is_0x01(&self) -> bool {
        *self == DLY_A::_0X01
    }
    #[doc = "Checks if the value of the field is `_0X02`"]
    #[inline(always)]
    pub fn is_0x02(&self) -> bool {
        *self == DLY_A::_0X02
    }
    #[doc = "Checks if the value of the field is `_0X03`"]
    #[inline(always)]
    pub fn is_0x03(&self) -> bool {
        *self == DLY_A::_0X03
    }
    #[doc = "Checks if the value of the field is `_0X04`"]
    #[inline(always)]
    pub fn is_0x04(&self) -> bool {
        *self == DLY_A::_0X04
    }
    #[doc = "Checks if the value of the field is `_0X05`"]
    #[inline(always)]
    pub fn is_0x05(&self) -> bool {
        *self == DLY_A::_0X05
    }
    #[doc = "Checks if the value of the field is `_0X06`"]
    #[inline(always)]
    pub fn is_0x06(&self) -> bool {
        *self == DLY_A::_0X06
    }
    #[doc = "Checks if the value of the field is `_0X07`"]
    #[inline(always)]
    pub fn is_0x07(&self) -> bool {
        *self == DLY_A::_0X07
    }
    #[doc = "Checks if the value of the field is `_0X08`"]
    #[inline(always)]
    pub fn is_0x08(&self) -> bool {
        *self == DLY_A::_0X08
    }
    #[doc = "Checks if the value of the field is `_0X09`"]
    #[inline(always)]
    pub fn is_0x09(&self) -> bool {
        *self == DLY_A::_0X09
    }
    #[doc = "Checks if the value of the field is `_0X0A`"]
    #[inline(always)]
    pub fn is_0x0a(&self) -> bool {
        *self == DLY_A::_0X0A
    }
    #[doc = "Checks if the value of the field is `_0X0B`"]
    #[inline(always)]
    pub fn is_0x0b(&self) -> bool {
        *self == DLY_A::_0X0B
    }
    #[doc = "Checks if the value of the field is `_0X0C`"]
    #[inline(always)]
    pub fn is_0x0c(&self) -> bool {
        *self == DLY_A::_0X0C
    }
    #[doc = "Checks if the value of the field is `_0X0D`"]
    #[inline(always)]
    pub fn is_0x0d(&self) -> bool {
        *self == DLY_A::_0X0D
    }
    #[doc = "Checks if the value of the field is `_0X0E`"]
    #[inline(always)]
    pub fn is_0x0e(&self) -> bool {
        *self == DLY_A::_0X0E
    }
    #[doc = "Checks if the value of the field is `_0X0F`"]
    #[inline(always)]
    pub fn is_0x0f(&self) -> bool {
        *self == DLY_A::_0X0F
    }
    #[doc = "Checks if the value of the field is `_0X10`"]
    #[inline(always)]
    pub fn is_0x10(&self) -> bool {
        *self == DLY_A::_0X10
    }
    #[doc = "Checks if the value of the field is `_0X11`"]
    #[inline(always)]
    pub fn is_0x11(&self) -> bool {
        *self == DLY_A::_0X11
    }
    #[doc = "Checks if the value of the field is `_0X12`"]
    #[inline(always)]
    pub fn is_0x12(&self) -> bool {
        *self == DLY_A::_0X12
    }
    #[doc = "Checks if the value of the field is `_0X13`"]
    #[inline(always)]
    pub fn is_0x13(&self) -> bool {
        *self == DLY_A::_0X13
    }
    #[doc = "Checks if the value of the field is `_0X14`"]
    #[inline(always)]
    pub fn is_0x14(&self) -> bool {
        *self == DLY_A::_0X14
    }
    #[doc = "Checks if the value of the field is `_0X15`"]
    #[inline(always)]
    pub fn is_0x15(&self) -> bool {
        *self == DLY_A::_0X15
    }
    #[doc = "Checks if the value of the field is `_0X16`"]
    #[inline(always)]
    pub fn is_0x16(&self) -> bool {
        *self == DLY_A::_0X16
    }
    #[doc = "Checks if the value of the field is `_0X17`"]
    #[inline(always)]
    pub fn is_0x17(&self) -> bool {
        *self == DLY_A::_0X17
    }
    #[doc = "Checks if the value of the field is `_0X18`"]
    #[inline(always)]
    pub fn is_0x18(&self) -> bool {
        *self == DLY_A::_0X18
    }
    #[doc = "Checks if the value of the field is `_0X19`"]
    #[inline(always)]
    pub fn is_0x19(&self) -> bool {
        *self == DLY_A::_0X19
    }
    #[doc = "Checks if the value of the field is `_0X1A`"]
    #[inline(always)]
    pub fn is_0x1a(&self) -> bool {
        *self == DLY_A::_0X1A
    }
    #[doc = "Checks if the value of the field is `_0X1B`"]
    #[inline(always)]
    pub fn is_0x1b(&self) -> bool {
        *self == DLY_A::_0X1B
    }
    #[doc = "Checks if the value of the field is `_0X1C`"]
    #[inline(always)]
    pub fn is_0x1c(&self) -> bool {
        *self == DLY_A::_0X1C
    }
    #[doc = "Checks if the value of the field is `_0X1D`"]
    #[inline(always)]
    pub fn is_0x1d(&self) -> bool {
        *self == DLY_A::_0X1D
    }
    #[doc = "Checks if the value of the field is `_0X1E`"]
    #[inline(always)]
    pub fn is_0x1e(&self) -> bool {
        *self == DLY_A::_0X1E
    }
    #[doc = "Checks if the value of the field is `_0X1F`"]
    #[inline(always)]
    pub fn is_0x1f(&self) -> bool {
        *self == DLY_A::_0X1F
    }
}
#[doc = "Field `DLY` writer - GTIOCnA Output Falling Edge Delay Setting"]
pub type DLY_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, GTDLYFA_SPEC, u8, DLY_A, 5, O>;
impl<'a, const O: u8> DLY_W<'a, O> {
    #[doc = "Delay on falling edges is not applied"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut W {
        self.variant(DLY_A::_0X00)
    }
    #[doc = "Delay of 1/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x01(self) -> &'a mut W {
        self.variant(DLY_A::_0X01)
    }
    #[doc = "Delay of 2/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x02(self) -> &'a mut W {
        self.variant(DLY_A::_0X02)
    }
    #[doc = "Delay of 3/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x03(self) -> &'a mut W {
        self.variant(DLY_A::_0X03)
    }
    #[doc = "Delay of 4/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x04(self) -> &'a mut W {
        self.variant(DLY_A::_0X04)
    }
    #[doc = "Delay of 5/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x05(self) -> &'a mut W {
        self.variant(DLY_A::_0X05)
    }
    #[doc = "Delay of 6/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x06(self) -> &'a mut W {
        self.variant(DLY_A::_0X06)
    }
    #[doc = "Delay of 7/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x07(self) -> &'a mut W {
        self.variant(DLY_A::_0X07)
    }
    #[doc = "Delay of 8/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x08(self) -> &'a mut W {
        self.variant(DLY_A::_0X08)
    }
    #[doc = "Delay of 9/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x09(self) -> &'a mut W {
        self.variant(DLY_A::_0X09)
    }
    #[doc = "Delay of 10/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x0a(self) -> &'a mut W {
        self.variant(DLY_A::_0X0A)
    }
    #[doc = "Delay of 11/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x0b(self) -> &'a mut W {
        self.variant(DLY_A::_0X0B)
    }
    #[doc = "Delay of 12/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x0c(self) -> &'a mut W {
        self.variant(DLY_A::_0X0C)
    }
    #[doc = "Delay of 13/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x0d(self) -> &'a mut W {
        self.variant(DLY_A::_0X0D)
    }
    #[doc = "Delay of 14/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x0e(self) -> &'a mut W {
        self.variant(DLY_A::_0X0E)
    }
    #[doc = "Delay of 15/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x0f(self) -> &'a mut W {
        self.variant(DLY_A::_0X0F)
    }
    #[doc = "Delay of 16/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x10(self) -> &'a mut W {
        self.variant(DLY_A::_0X10)
    }
    #[doc = "Delay of 17/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x11(self) -> &'a mut W {
        self.variant(DLY_A::_0X11)
    }
    #[doc = "Delay of 18/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x12(self) -> &'a mut W {
        self.variant(DLY_A::_0X12)
    }
    #[doc = "Delay of 19/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x13(self) -> &'a mut W {
        self.variant(DLY_A::_0X13)
    }
    #[doc = "Delay of 20/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x14(self) -> &'a mut W {
        self.variant(DLY_A::_0X14)
    }
    #[doc = "Delay of 21/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x15(self) -> &'a mut W {
        self.variant(DLY_A::_0X15)
    }
    #[doc = "Delay of 22/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x16(self) -> &'a mut W {
        self.variant(DLY_A::_0X16)
    }
    #[doc = "Delay of 23/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x17(self) -> &'a mut W {
        self.variant(DLY_A::_0X17)
    }
    #[doc = "Delay of 24/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x18(self) -> &'a mut W {
        self.variant(DLY_A::_0X18)
    }
    #[doc = "Delay of 25/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x19(self) -> &'a mut W {
        self.variant(DLY_A::_0X19)
    }
    #[doc = "Delay of 26/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x1a(self) -> &'a mut W {
        self.variant(DLY_A::_0X1A)
    }
    #[doc = "Delay of 27/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x1b(self) -> &'a mut W {
        self.variant(DLY_A::_0X1B)
    }
    #[doc = "Delay of 28/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x1c(self) -> &'a mut W {
        self.variant(DLY_A::_0X1C)
    }
    #[doc = "Delay of 29/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x1d(self) -> &'a mut W {
        self.variant(DLY_A::_0X1D)
    }
    #[doc = "Delay of 30/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x1e(self) -> &'a mut W {
        self.variant(DLY_A::_0X1E)
    }
    #[doc = "Delay of 31/32 times GTCLK period applied"]
    #[inline(always)]
    pub fn _0x1f(self) -> &'a mut W {
        self.variant(DLY_A::_0X1F)
    }
}
impl R {
    #[doc = "Bits 0:4 - GTIOCnA Output Falling Edge Delay Setting"]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - GTIOCnA Output Falling Edge Delay Setting"]
    #[inline(always)]
    #[must_use]
    pub fn dly(&mut self) -> DLY_W<0> {
        DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTIOCnA Falling Output Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtdlyfa](index.html) module"]
pub struct GTDLYFA_SPEC;
impl crate::RegisterSpec for GTDLYFA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [gtdlyfa::R](R) reader structure"]
impl crate::Readable for GTDLYFA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtdlyfa::W](W) writer structure"]
impl crate::Writable for GTDLYFA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTDLYF%sA to value 0"]
impl crate::Resettable for GTDLYFA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
