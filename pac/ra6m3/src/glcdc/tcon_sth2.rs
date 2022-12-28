#[doc = "Register `TCON_STH%s2` reader"]
pub struct R(crate::R<TCON_STH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCON_STH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCON_STH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCON_STH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCON_STH%s2` writer"]
pub struct W(crate::W<TCON_STH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCON_STH2_SPEC>;
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
impl From<crate::W<TCON_STH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCON_STH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Output signal select control for LCD_TCON2 (controlled by TCON_STHA2 register)/LCD_TCON3 (controlled by the TCON_STHB2 register) pin."]
pub type SEL_R = crate::FieldReader<u8, SEL_A>;
#[doc = "Output signal select control for LCD_TCON2 (controlled by TCON_STHA2 register)/LCD_TCON3 (controlled by the TCON_STHB2 register) pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "7: DE"]
    _111 = 7,
    #[doc = "6: Setting prohibited"]
    _110 = 6,
    #[doc = "5: Setting prohibited"]
    _101 = 5,
    #[doc = "4: Setting prohibited"]
    _100 = 4,
    #[doc = "3: STHB"]
    _011 = 3,
    #[doc = "2: STHA"]
    _010 = 2,
    #[doc = "1: STVB"]
    _001 = 1,
    #[doc = "0: STVA"]
    _000 = 0,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
impl SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            7 => SEL_A::_111,
            6 => SEL_A::_110,
            5 => SEL_A::_101,
            4 => SEL_A::_100,
            3 => SEL_A::_011,
            2 => SEL_A::_010,
            1 => SEL_A::_001,
            0 => SEL_A::_000,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SEL_A::_111
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SEL_A::_110
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SEL_A::_001
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SEL_A::_000
    }
}
#[doc = "Field `SEL` writer - Output signal select control for LCD_TCON2 (controlled by TCON_STHA2 register)/LCD_TCON3 (controlled by the TCON_STHB2 register) pin."]
pub type SEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TCON_STH2_SPEC, u8, SEL_A, 3, O>;
impl<'a, const O: u8> SEL_W<'a, O> {
    #[doc = "DE"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SEL_A::_111)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SEL_A::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SEL_A::_101)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SEL_A::_100)
    }
    #[doc = "STHB"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SEL_A::_011)
    }
    #[doc = "STHA"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SEL_A::_010)
    }
    #[doc = "STVB"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SEL_A::_001)
    }
    #[doc = "STVA"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SEL_A::_000)
    }
}
#[doc = "Field `INV` reader - STVx signal polarity inversion control."]
pub type INV_R = crate::BitReader<INV_A>;
#[doc = "STVx signal polarity inversion control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INV_A {
    #[doc = "1: Inverted"]
    _1 = 1,
    #[doc = "0: Not inverted"]
    _0 = 0,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
impl INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            true => INV_A::_1,
            false => INV_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV_A::_0
    }
}
#[doc = "Field `INV` writer - STVx signal polarity inversion control."]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCON_STH2_SPEC, INV_A, O>;
impl<'a, const O: u8> INV_W<'a, O> {
    #[doc = "Inverted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV_A::_1)
    }
    #[doc = "Not inverted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV_A::_0)
    }
}
#[doc = "Field `HSSEL` reader - STHx signal generation reference timing control."]
pub type HSSEL_R = crate::BitReader<HSSEL_A>;
#[doc = "STHx signal generation reference timing control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSSEL_A {
    #[doc = "1: Reference timing is the offset set with the TCON_TIM.OFFSET\\[10:0\\]
(horizontal synchronization generation reference timing) field"]
    _1 = 1,
    #[doc = "0: Reference timing is the input horizontal synchronization signal (HSIN)"]
    _0 = 0,
}
impl From<HSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HSSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl HSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSSEL_A {
        match self.bits {
            true => HSSEL_A::_1,
            false => HSSEL_A::_0,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSSEL_A::_0
    }
}
#[doc = "Field `HSSEL` writer - STHx signal generation reference timing control."]
pub type HSSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCON_STH2_SPEC, HSSEL_A, O>;
impl<'a, const O: u8> HSSEL_W<'a, O> {
    #[doc = "Reference timing is the offset set with the TCON_TIM.OFFSET\\[10:0\\]
(horizontal synchronization generation reference timing) field"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSSEL_A::_1)
    }
    #[doc = "Reference timing is the input horizontal synchronization signal (HSIN)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSSEL_A::_0)
    }
}
impl R {
    #[doc = "Bits 0:2 - Output signal select control for LCD_TCON2 (controlled by TCON_STHA2 register)/LCD_TCON3 (controlled by the TCON_STHB2 register) pin."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - STVx signal polarity inversion control."]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - STHx signal generation reference timing control."]
    #[inline(always)]
    pub fn hssel(&self) -> HSSEL_R {
        HSSEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Output signal select control for LCD_TCON2 (controlled by TCON_STHA2 register)/LCD_TCON3 (controlled by the TCON_STHB2 register) pin."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Bit 4 - STVx signal polarity inversion control."]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<4> {
        INV_W::new(self)
    }
    #[doc = "Bit 8 - STHx signal generation reference timing control."]
    #[inline(always)]
    #[must_use]
    pub fn hssel(&mut self) -> HSSEL_W<8> {
        HSSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCON Horizontal Timing Setting Register STH%s2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcon_sth2](index.html) module"]
pub struct TCON_STH2_SPEC;
impl crate::RegisterSpec for TCON_STH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcon_sth2::R](R) reader structure"]
impl crate::Readable for TCON_STH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcon_sth2::W](W) writer structure"]
impl crate::Writable for TCON_STH2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCON_STH%s2 to value 0"]
impl crate::Resettable for TCON_STH2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
