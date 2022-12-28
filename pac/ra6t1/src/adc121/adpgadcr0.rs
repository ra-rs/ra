#[doc = "Register `ADPGADCR0` reader"]
pub struct R(crate::R<ADPGADCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPGADCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPGADCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPGADCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADPGADCR0` writer"]
pub struct W(crate::W<ADPGADCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPGADCR0_SPEC>;
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
impl From<crate::W<ADPGADCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPGADCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P000DG` reader - P000 Differential Input Gain Setting NOTE: When these bits are used, set {P000DEN, P000GEN} to 11b."]
pub type P000DG_R = crate::FieldReader<u8, P000DG_A>;
#[doc = "P000 Differential Input Gain Setting NOTE: When these bits are used, set {P000DEN, P000GEN} to 11b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P000DG_A {
    #[doc = "0: x 1.5"]
    _00 = 0,
    #[doc = "1: x 2.333"]
    _01 = 1,
    #[doc = "2: x 4.0"]
    _10 = 2,
    #[doc = "3: x 5.667"]
    _11 = 3,
}
impl From<P000DG_A> for u8 {
    #[inline(always)]
    fn from(variant: P000DG_A) -> Self {
        variant as _
    }
}
impl P000DG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P000DG_A {
        match self.bits {
            0 => P000DG_A::_00,
            1 => P000DG_A::_01,
            2 => P000DG_A::_10,
            3 => P000DG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == P000DG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == P000DG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == P000DG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == P000DG_A::_11
    }
}
#[doc = "Field `P000DG` writer - P000 Differential Input Gain Setting NOTE: When these bits are used, set {P000DEN, P000GEN} to 11b."]
pub type P000DG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADPGADCR0_SPEC, u8, P000DG_A, 2, O>;
impl<'a, const O: u8> P000DG_W<'a, O> {
    #[doc = "x 1.5"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(P000DG_A::_00)
    }
    #[doc = "x 2.333"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(P000DG_A::_01)
    }
    #[doc = "x 4.0"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(P000DG_A::_10)
    }
    #[doc = "x 5.667"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(P000DG_A::_11)
    }
}
#[doc = "Field `P000DEN` reader - P000 Differential Input Enable"]
pub type P000DEN_R = crate::BitReader<P000DEN_A>;
#[doc = "P000 Differential Input Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P000DEN_A {
    #[doc = "0: Differential input is disabled."]
    _0 = 0,
    #[doc = "1: Differential input is enabled."]
    _1 = 1,
}
impl From<P000DEN_A> for bool {
    #[inline(always)]
    fn from(variant: P000DEN_A) -> Self {
        variant as u8 != 0
    }
}
impl P000DEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P000DEN_A {
        match self.bits {
            false => P000DEN_A::_0,
            true => P000DEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P000DEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P000DEN_A::_1
    }
}
#[doc = "Field `P000DEN` writer - P000 Differential Input Enable"]
pub type P000DEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGADCR0_SPEC, P000DEN_A, O>;
impl<'a, const O: u8> P000DEN_W<'a, O> {
    #[doc = "Differential input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P000DEN_A::_0)
    }
    #[doc = "Differential input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P000DEN_A::_1)
    }
}
#[doc = "Field `P001DG` reader - P001 Differential Input Gain Setting NOTE: When these bits are used, set {P001DEN, P001GEN} to 11b."]
pub type P001DG_R = crate::FieldReader<u8, P001DG_A>;
#[doc = "P001 Differential Input Gain Setting NOTE: When these bits are used, set {P001DEN, P001GEN} to 11b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P001DG_A {
    #[doc = "0: x 1.5"]
    _00 = 0,
    #[doc = "1: x 2.333"]
    _01 = 1,
    #[doc = "2: x 4.0"]
    _10 = 2,
    #[doc = "3: x 5.667"]
    _11 = 3,
}
impl From<P001DG_A> for u8 {
    #[inline(always)]
    fn from(variant: P001DG_A) -> Self {
        variant as _
    }
}
impl P001DG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P001DG_A {
        match self.bits {
            0 => P001DG_A::_00,
            1 => P001DG_A::_01,
            2 => P001DG_A::_10,
            3 => P001DG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == P001DG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == P001DG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == P001DG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == P001DG_A::_11
    }
}
#[doc = "Field `P001DG` writer - P001 Differential Input Gain Setting NOTE: When these bits are used, set {P001DEN, P001GEN} to 11b."]
pub type P001DG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADPGADCR0_SPEC, u8, P001DG_A, 2, O>;
impl<'a, const O: u8> P001DG_W<'a, O> {
    #[doc = "x 1.5"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(P001DG_A::_00)
    }
    #[doc = "x 2.333"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(P001DG_A::_01)
    }
    #[doc = "x 4.0"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(P001DG_A::_10)
    }
    #[doc = "x 5.667"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(P001DG_A::_11)
    }
}
#[doc = "Field `P001DEN` reader - P001 Differential Input Enable"]
pub type P001DEN_R = crate::BitReader<P001DEN_A>;
#[doc = "P001 Differential Input Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P001DEN_A {
    #[doc = "0: Differential input is disabled."]
    _0 = 0,
    #[doc = "1: Differential input is enabled."]
    _1 = 1,
}
impl From<P001DEN_A> for bool {
    #[inline(always)]
    fn from(variant: P001DEN_A) -> Self {
        variant as u8 != 0
    }
}
impl P001DEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P001DEN_A {
        match self.bits {
            false => P001DEN_A::_0,
            true => P001DEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P001DEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P001DEN_A::_1
    }
}
#[doc = "Field `P001DEN` writer - P001 Differential Input Enable"]
pub type P001DEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGADCR0_SPEC, P001DEN_A, O>;
impl<'a, const O: u8> P001DEN_W<'a, O> {
    #[doc = "Differential input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P001DEN_A::_0)
    }
    #[doc = "Differential input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P001DEN_A::_1)
    }
}
#[doc = "Field `P002DG` reader - P002 Differential Input Gain Setting NOTE: When these bits are used, set {P002DEN, P002GEN} to 11b."]
pub type P002DG_R = crate::FieldReader<u8, P002DG_A>;
#[doc = "P002 Differential Input Gain Setting NOTE: When these bits are used, set {P002DEN, P002GEN} to 11b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P002DG_A {
    #[doc = "0: x 1.5"]
    _00 = 0,
    #[doc = "1: x 2.333"]
    _01 = 1,
    #[doc = "2: x 4.0"]
    _10 = 2,
    #[doc = "3: x 5.667"]
    _11 = 3,
}
impl From<P002DG_A> for u8 {
    #[inline(always)]
    fn from(variant: P002DG_A) -> Self {
        variant as _
    }
}
impl P002DG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P002DG_A {
        match self.bits {
            0 => P002DG_A::_00,
            1 => P002DG_A::_01,
            2 => P002DG_A::_10,
            3 => P002DG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == P002DG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == P002DG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == P002DG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == P002DG_A::_11
    }
}
#[doc = "Field `P002DG` writer - P002 Differential Input Gain Setting NOTE: When these bits are used, set {P002DEN, P002GEN} to 11b."]
pub type P002DG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADPGADCR0_SPEC, u8, P002DG_A, 2, O>;
impl<'a, const O: u8> P002DG_W<'a, O> {
    #[doc = "x 1.5"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(P002DG_A::_00)
    }
    #[doc = "x 2.333"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(P002DG_A::_01)
    }
    #[doc = "x 4.0"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(P002DG_A::_10)
    }
    #[doc = "x 5.667"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(P002DG_A::_11)
    }
}
#[doc = "Field `P002DEN` reader - P002 Differential Input Enable"]
pub type P002DEN_R = crate::BitReader<P002DEN_A>;
#[doc = "P002 Differential Input Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P002DEN_A {
    #[doc = "0: Differential input is disabled."]
    _0 = 0,
    #[doc = "1: Differential input is enabled."]
    _1 = 1,
}
impl From<P002DEN_A> for bool {
    #[inline(always)]
    fn from(variant: P002DEN_A) -> Self {
        variant as u8 != 0
    }
}
impl P002DEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P002DEN_A {
        match self.bits {
            false => P002DEN_A::_0,
            true => P002DEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == P002DEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == P002DEN_A::_1
    }
}
#[doc = "Field `P002DEN` writer - P002 Differential Input Enable"]
pub type P002DEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADPGADCR0_SPEC, P002DEN_A, O>;
impl<'a, const O: u8> P002DEN_W<'a, O> {
    #[doc = "Differential input is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(P002DEN_A::_0)
    }
    #[doc = "Differential input is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(P002DEN_A::_1)
    }
}
#[doc = "Field `P003DG` reader - P003 Differential Input Gain Setting NOTE: When these bits are used, set {P003DEN, P003GEN} to 11b."]
pub type P003DG_R = crate::FieldReader<u8, P003DG_A>;
#[doc = "P003 Differential Input Gain Setting NOTE: When these bits are used, set {P003DEN, P003GEN} to 11b.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P003DG_A {
    #[doc = "0: x 1.5"]
    _00 = 0,
    #[doc = "1: x 2.333"]
    _01 = 1,
    #[doc = "2: x 4.0"]
    _10 = 2,
    #[doc = "3: x 5.667"]
    _11 = 3,
}
impl From<P003DG_A> for u8 {
    #[inline(always)]
    fn from(variant: P003DG_A) -> Self {
        variant as _
    }
}
impl P003DG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> P003DG_A {
        match self.bits {
            0 => P003DG_A::_00,
            1 => P003DG_A::_01,
            2 => P003DG_A::_10,
            3 => P003DG_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == P003DG_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == P003DG_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == P003DG_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == P003DG_A::_11
    }
}
#[doc = "Field `P003DG` writer - P003 Differential Input Gain Setting NOTE: When these bits are used, set {P003DEN, P003GEN} to 11b."]
pub type P003DG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADPGADCR0_SPEC, u8, P003DG_A, 2, O>;
impl<'a, const O: u8> P003DG_W<'a, O> {
    #[doc = "x 1.5"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(P003DG_A::_00)
    }
    #[doc = "x 2.333"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(P003DG_A::_01)
    }
    #[doc = "x 4.0"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(P003DG_A::_10)
    }
    #[doc = "x 5.667"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(P003DG_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - P000 Differential Input Gain Setting NOTE: When these bits are used, set {P000DEN, P000GEN} to 11b."]
    #[inline(always)]
    pub fn p000dg(&self) -> P000DG_R {
        P000DG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - P000 Differential Input Enable"]
    #[inline(always)]
    pub fn p000den(&self) -> P000DEN_R {
        P000DEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - P001 Differential Input Gain Setting NOTE: When these bits are used, set {P001DEN, P001GEN} to 11b."]
    #[inline(always)]
    pub fn p001dg(&self) -> P001DG_R {
        P001DG_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - P001 Differential Input Enable"]
    #[inline(always)]
    pub fn p001den(&self) -> P001DEN_R {
        P001DEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - P002 Differential Input Gain Setting NOTE: When these bits are used, set {P002DEN, P002GEN} to 11b."]
    #[inline(always)]
    pub fn p002dg(&self) -> P002DG_R {
        P002DG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - P002 Differential Input Enable"]
    #[inline(always)]
    pub fn p002den(&self) -> P002DEN_R {
        P002DEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - P003 Differential Input Gain Setting NOTE: When these bits are used, set {P003DEN, P003GEN} to 11b."]
    #[inline(always)]
    pub fn p003dg(&self) -> P003DG_R {
        P003DG_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - P000 Differential Input Gain Setting NOTE: When these bits are used, set {P000DEN, P000GEN} to 11b."]
    #[inline(always)]
    #[must_use]
    pub fn p000dg(&mut self) -> P000DG_W<0> {
        P000DG_W::new(self)
    }
    #[doc = "Bit 3 - P000 Differential Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p000den(&mut self) -> P000DEN_W<3> {
        P000DEN_W::new(self)
    }
    #[doc = "Bits 4:5 - P001 Differential Input Gain Setting NOTE: When these bits are used, set {P001DEN, P001GEN} to 11b."]
    #[inline(always)]
    #[must_use]
    pub fn p001dg(&mut self) -> P001DG_W<4> {
        P001DG_W::new(self)
    }
    #[doc = "Bit 7 - P001 Differential Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p001den(&mut self) -> P001DEN_W<7> {
        P001DEN_W::new(self)
    }
    #[doc = "Bits 8:9 - P002 Differential Input Gain Setting NOTE: When these bits are used, set {P002DEN, P002GEN} to 11b."]
    #[inline(always)]
    #[must_use]
    pub fn p002dg(&mut self) -> P002DG_W<8> {
        P002DG_W::new(self)
    }
    #[doc = "Bit 11 - P002 Differential Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p002den(&mut self) -> P002DEN_W<11> {
        P002DEN_W::new(self)
    }
    #[doc = "Bits 12:13 - P003 Differential Input Gain Setting NOTE: When these bits are used, set {P003DEN, P003GEN} to 11b."]
    #[inline(always)]
    #[must_use]
    pub fn p003dg(&mut self) -> P003DG_W<12> {
        P003DG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Programmable Gain Amplifier Differential Input Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpgadcr0](index.html) module"]
pub struct ADPGADCR0_SPEC;
impl crate::RegisterSpec for ADPGADCR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adpgadcr0::R](R) reader structure"]
impl crate::Readable for ADPGADCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpgadcr0::W](W) writer structure"]
impl crate::Writable for ADPGADCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADPGADCR0 to value 0x8888"]
impl crate::Resettable for ADPGADCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x8888;
}
